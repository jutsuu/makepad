
use crate::LiveId;
use crate::event::HttpRequest;
use crate::event::{NetworkResponseItem,NetworkResponse, HttpResponse};
use std::sync::mpsc::{Sender};
use makepad_futures_legacy::executor;

use windows::{
    core::*,
    Web::Http::{HttpClient, IHttpContent,  HttpStreamContent, HttpCompletionOption, HttpMethod, HttpRequestMessage},
    Foundation::Uri,
    Win32::System::WinRT::IBufferByteAccess,
    Storage::Streams::{DataWriter, Buffer, InputStreamOptions, InMemoryRandomAccessStream},
};

pub struct WindowsHttpSocket{
    //sender: Option<Sender<Vec<u8>>>
}

impl WindowsHttpSocket{
    /*pub fn send_message(&mut self, message:WebSocketMessage)->Result<(),()>{
        // lets encode the message into a membuffer and send it to the write thread
        if let Some(sender) = &mut self.sender{
            if sender.send(message).is_err(){
                return Err(());
            }
            return Ok(())
        }
        Err(())
    }*/
                        
    pub fn open(request_id:LiveId, request: HttpRequest, response_sender:Sender<NetworkResponseItem>){
        // parse the url
        
        async fn create_request(request: &HttpRequest) -> windows::core::Result<HttpRequestMessage> {
            
            let uri = Uri::CreateUri(&request.url.to_string().into())?;
            let req = HttpRequestMessage::Create(&HttpMethod::Create(&request.method.to_string().into())?, &uri)?;
                        
            let headers_map = req.Headers()?;
            for (key, values) in request.headers.iter() {
                for value in values {
                    headers_map.Append(&key.into(), &value.into())?;
                }
            }
                        
            // lets set the body
            if let Some(body) = &request.body{
                let stream = InMemoryRandomAccessStream::new()?;
                let writer = DataWriter::CreateDataWriter(&stream.GetOutputStreamAt(0)?)?;
                // Write the bytes to the stream
                writer.WriteBytes(&body)?;
                writer.StoreAsync()?.await?;
                writer.FlushAsync()?.await?;
                // Reset stream position to beginning
                stream.Seek(0)?;
                                        
                // Create and set content
                let content = HttpStreamContent::CreateFromInputStream(&stream)?;
                req.SetContent(&content.cast::<IHttpContent>()?)?;
            }
            
            Ok(req)
        }        
                
        async fn streaming_request(request_id:LiveId, request: HttpRequest, response_sender:Sender<NetworkResponseItem>) -> windows::core::Result<()> {
            let client = HttpClient::new()?;
            let req =  create_request(&request).await?;
            let response = client.SendRequestWithOptionAsync(&req, HttpCompletionOption::ResponseHeadersRead)?.await?;
            
            let input_stream = response.Content()?.ReadAsInputStreamAsync()?.await?;
            let buffer = Buffer::Create(1024*1024)?; // 1MB chunks
            loop {
                // Read data into buffer
                input_stream
                .ReadAsync(&buffer, buffer.Capacity()?, InputStreamOptions::Partial)?
                .await?;
                // Process the chunk of data
                let chunk_size = buffer.Length()?;
                // ok we got a buffer, lets emit i
                if chunk_size == 0{
                    break;
                }
                let byte_access:IBufferByteAccess = buffer.cast()?;
                // Get pointer to the buffer's data
                let chunk =  unsafe {std::slice::from_raw_parts(byte_access.as_raw() as *const u8, chunk_size as usize)};
                let message = NetworkResponseItem {
                    request_id: request_id,
                    response: NetworkResponse::HttpStreamResponse(HttpResponse{
                        headers: Default::default(),
                        metadata_id: request.metadata_id,
                        status_code: 0,
                        body:Some(chunk.to_vec())
                    }),
                };
                response_sender.send(message).unwrap();
            }
            let message = NetworkResponseItem {
                request_id: request_id,
                response: NetworkResponse::HttpStreamComplete(HttpResponse{
                    headers: Default::default(),
                    metadata_id: request.metadata_id,
                    status_code: 0,
                    body:None
                }),
            };
            response_sender.send(message).unwrap();
            Ok(())
        }
        
        async fn non_streaming_request(request_id:LiveId, request: HttpRequest, response_sender:Sender<NetworkResponseItem>) -> windows::core::Result<()> {
            let client = HttpClient::new()?;
            let req =  create_request(&request).await?;
            let response = client.SendRequestWithOptionAsync(&req, HttpCompletionOption::ResponseHeadersRead)?.await?;
            
            let buffer = response.Content()?.ReadAsBufferAsync()?.await?;
            let byte_access:IBufferByteAccess = buffer.cast()?;
            let chunk_size = buffer.Length()?;
            let chunk =  unsafe {std::slice::from_raw_parts(byte_access.as_raw() as *const u8, chunk_size as usize)};
            let message = NetworkResponseItem {
                request_id: request_id,
                response: NetworkResponse::HttpResponse(HttpResponse{
                    headers: Default::default(),
                    metadata_id: request.metadata_id,
                    status_code: 0,
                    body:Some(chunk.to_vec())
                }),
            };
            response_sender.send(message).unwrap();
            Ok(())
        }
        
                
        // create a thread and run the request
        let _reader_thread = std::thread::spawn(move || {
           if request.is_streaming{
               let _ = executor::block_on(streaming_request(request_id, request, response_sender));
           }
           else{
               let _ = executor::block_on(non_streaming_request(request_id, request, response_sender));
           }
        });
        /*
        let split = request.split_url();
        // strip off any hashes
        // alright we have proto, host, port and hash now
        // lets open a tcpstream
        let stream = TcpStream::connect(format!("{}:{}", split.host, split.port));
        // alright lets construct a http request
        // lets join the headers
                
        let mut http_header = format!("{} /{} HTTP/1.1\r\nHost: {}\r\n", request.method.to_string(), split.file, split.host);
        http_header.push_str(&request.get_headers_string());
        http_header.push_str("\r\n"); 
        println!("Sending headers #{}#", http_header);

        // lets push the entire body
        // lets write the http request
        if stream.is_err(){
            response_sender.send(NetworkResponseItem{
                request_id: request_id,
                response: NetworkResponse::HttpResponse(HttpResponse{
                    metadata_id: request.metadata_id,
                    status_code: 400,
                    headers: Default::default(),
                    body:Some(Vec::new())
                })
            }).ok();
            return
        }
        let mut stream = stream.unwrap();
        if write_bytes_to_tcp_stream_no_error(&mut stream, http_header.as_bytes()){
            response_sender.send(NetworkResponseItem{
                request_id: request_id,
                response: NetworkResponse::HttpResponse(HttpResponse{
                    metadata_id: request.metadata_id,
                    status_code: 400,
                    headers: Default::default(),
                    body:Some(Vec::new())
                })
            }).ok();
            return
        }
        
        if let Some(body) = request.body{
            println!("SENDING BODY {}", body.len());
            if write_bytes_to_tcp_stream_no_error(&mut stream, &body){
                response_sender.send(NetworkResponseItem{
                    request_id: request_id,
                    response: NetworkResponse::HttpResponse(HttpResponse{
                        metadata_id: request.metadata_id,
                        status_code: 400,
                        headers: Default::default(),
                        body:Some(Vec::new())
                    })
                }).ok();
                return
            }
        }
        else{
            println!("NO BODY");
        }
        
        let _reader_thread = std::thread::spawn(move || {
            let mut ret_buf = Vec::new();
            loop {
                let mut buffer = [0u8; 65535];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0{
                            break;
                        }
                        ret_buf.extend_from_slice(&buffer[0..bytes_read]);
                    },
                    Err(_)=>{
                        break;
                    }
                }
            }
            // alright we have a ret_buf, now we need to split it at \r\n\r\n and return it
            println!("GOT RET {}", std::str::from_utf8(&ret_buf).unwrap_or(""));
            response_sender.send(NetworkResponseItem{
                request_id: request_id,
                response: NetworkResponse::HttpResponse(HttpResponse{
                    metadata_id: request.metadata_id,
                    status_code: 200,
                    headers: Default::default(),
                    body:Some(Vec::new())
                })
            }).ok();
        });
        */
        /*
        // lets start the read thread
        let mut input_stream = stream.try_clone().unwrap();
        let mut output_stream = stream.try_clone().unwrap();
        let (sender, receiver) = channel();
        
        let _writer_thread = std::thread::spawn(move || {
        });
                        
        OsWebSocket{sender:Some(sender)}
        */
    }
}