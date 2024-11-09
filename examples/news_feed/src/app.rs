use makepad_widgets::*;
   
live_design!{ 
	import makepad_widgets::base::*;
	import makepad_widgets::theme_desktop_dark::*;
	import makepad_draw::shader::std::*;
	IMG_A = dep("crate://self/resources/neom-THlO6Mkf5uI-unsplash.jpg")
	IMG_PROFILE_A = dep("crate://self/resources/profile_1.jpg")
	IMG_PROFILE_B = dep("crate://self/resources/profile_2.jpg")
	LOGO = dep("crate://self/resources/logo.svg")
	ICO_FAV = dep("crate://self/resources/icon_favorite.svg")
	ICO_COMMENT = dep("crate://self/resources/icon_comment.svg")
	ICO_REPLY = dep("crate://self/resources/icon_reply.svg")
	ICO_HOME = dep("crate://self/resources/icon_home.svg")
	ICO_FIND = dep("crate://self/resources/icon_find.svg")
	ICO_LIKES = dep("crate://self/resources/icon_likes.svg")
	ICO_USER = dep("crate://self/resources/icon_user.svg")
	ICO_ADD = dep("crate://self/resources/icon_add.svg")
	
	FONT_SIZE_SUB = 10.5
	FONT_SIZE_P = 10.5
	
	TEXT_SUB = {
		font_size: (FONT_SIZE_SUB),
		font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
	}
	
	TEXT_P = {
		font_size: (FONT_SIZE_P),
		height_factor: 1.65,
		font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
	}
	
	COLOR_BG = #xECECEC
	COLOR_HOVER = #x0CF
	COLOR_PRESSED = #x08F
	COLOR_META_TEXT = #xCCC
	COLOR_USER = #x333
	COLOR_META = #xaaa
	COLOR_META_INV = #xfffa
	COLOR_OVERLAY_BG = #xFFFFFFFF
	COLOR_DIVIDER = #x00000010
	COLOR_PROFILE_CIRCLE = #xFFFFFF
	COLOR_P = #x999
	
	Filler = <View> {width: Fill, height: Fill}
	
	Logo = <Button> {
		draw_icon: {
			svg_file: (LOGO),
			fn get_color(self) -> vec4 {
				return (COLOR_HOVER)
			}
		}
		icon_walk: {width: 7.5, height: Fit}
		draw_bg: {
			fn pixel(self) -> vec4 {
				let sdf = Sdf2d::viewport(self.pos * self.rect_size);
				return sdf.result
			}
		}
		padding: 9.0
		text: ""
	}
	
	IconButton = <Button> {
		draw_text: {
			instance hover: 0.0
			instance pressed: 0.0
			text_style: {
				font_size: 11.0
			}
			fn get_color(self) -> vec4 {
				return mix(
					mix(
						(COLOR_META_TEXT),
						(COLOR_HOVER),
						self.hover
					),
					(COLOR_PRESSED),
					self.pressed
				)
			}
		}
		draw_icon: {
			svg_file: (ICO_FAV),
			fn get_color(self) -> vec4 {
				return mix(
					mix(
						(COLOR_META),
						(COLOR_HOVER),
						self.hover
					),
					(COLOR_PRESSED),
					self.pressed
				)
			}
		}
		icon_walk: {width: 7.5, height: Fit, margin: {left: 5.0}}
		draw_bg: {
			fn pixel(self) -> vec4 {
				let sdf = Sdf2d::viewport(self.pos * self.rect_size);
				return sdf.result
			}
		}
		padding: 9.0
		text: "1"
	}
	
	
	Header = <View> {
		width: Fill,
		height: Fit,
		flow: Down,
		show_bg: true,
		draw_bg: {color: (COLOR_BG)}

		<View> {
			width: Fill,
			height: 60
			flow: Right,
			padding: 10.0,
			spacing: 10.0
			align: { x: 0., y: 0.5}
			
			<Logo> {
				height: Fit,
				width: Fit,
				icon_walk: {width: Fit, height: 20.0}
			}
			
			<Image> {
				source: (IMG_PROFILE_A)
				margin: 0,
				width: 30., height: 30.
				draw_bg: {
					fn pixel(self) -> vec4 {
						let sdf = Sdf2d::viewport(self.pos * self.rect_size);
						let c = self.rect_size * 0.5;
						sdf.circle(c.x, c.y, c.x - 2.)
						sdf.fill_keep(self.get_color());
						sdf.stroke((COLOR_PROFILE_CIRCLE), 1);
						return sdf.result
					}
				}
			}

			<TextInput> {
				width: Fill
				empty_message: "What are you doing currently?",
			
				draw_bg: {
					instance radius: (THEME_CORNER_RADIUS)
					instance hover: 0.0
					instance focus: 0.0
					instance bodytop: #00000000
					instance bodybottom: #00000010

					fn pixel(self) -> vec4 {
						let sdf = Sdf2d::viewport(self.pos * self.rect_size);
						let grad_top = 5.0;
						let grad_bot = 1.5;

						let body = mix(
							self.bodytop,
							self.bodybottom,
							self.focus
						);

						let body_transp = (THEME_COLOR_D_HIDDEN)

						let top_gradient = mix(
							body_transp,
							#00000000,
							max(0.0, grad_top - sdf.pos.y) / grad_top
						);

						let bot_gradient = mix(
							(THEME_COLOR_BEVEL_LIGHT),
							top_gradient,
							clamp((self.rect_size.y - grad_bot - sdf.pos.y - 1.0) / grad_bot, 0.0, 1.0)
						);

						sdf.box(
							1.,
							1.,
							self.rect_size.x - 2.0,
							self.rect_size.y - 2.0,
							self.radius
						)

						sdf.fill_keep(body)

						sdf.stroke(
							bot_gradient,
							THEME_BEVELING * 0.9
						)

						return sdf.result
					}
				}
				
				draw_text: {
					instance hover: 0.0
					instance focus: 0.0
					wrap: Word,
					text_style: <THEME_FONT_REGULAR> {
						line_spacing: (THEME_FONT_LINE_SPACING),
						font_size: (THEME_FONT_SIZE_P)
					}
					fn get_color(self) -> vec4 {
						return
						mix(
							mix(#00000099, #000000CC, self.hover),
							mix(#00000044, #00000055, self.hover),
							self.is_empty
						)
					}
				}
			}

			<IconButton> {
				draw_icon: {svg_file: (ICO_FIND)},
				icon_walk: {width: 16.0, height: Fit},
				text: ""
				margin: { right: -20. }
			}
			<IconButton> {
				draw_icon: {svg_file: (ICO_ADD)},
				icon_walk: {width: 18.0, height: Fit},
				text: ""
			}
		}
	
	}
	
	Menu = <View> {
		width: Fill,
		height: 80
		flow: Right,
		padding: 10.0,
		spacing: 10.0
		
		show_bg: true,
		draw_bg: {color: (COLOR_BG)}
		
		<View> {
			width: Fill,
			height: Fit,
			margin: 0.0
			flow: Right,
			padding: 0.0,
			align: {x: 0.5, y: 0.5}
			
			<Filler> {}
			<IconButton> {draw_icon: {svg_file: (ICO_HOME)} icon_walk: {width: 23.0, height: Fit}, text: ""}
			<Filler> {}
			<IconButton> {draw_icon: {svg_file: (ICO_LIKES)} icon_walk: {width: 20.0, height: Fit}, text: ""}
			<Filler> {}
			<IconButton> {draw_icon: {svg_file: (ICO_USER)} icon_walk: {width: 15.0, height: Fit}, text: ""}
			<Filler> {}
		}
	}
	
	LineH = <RoundedView> {
		width: Fill,
		height: 2,
		margin: 0.0
		padding: 0.0,
		spacing: 0.0
		draw_bg: {color: (COLOR_DIVIDER)}
	}
	
	PostMenu = <View> {
		width: Fit, height: Fit,
		margin: 0.0
		padding: 0.0,
		flow: Down,
		spacing: 0.0
		
		<View> {
			width: Fit, height: Fit,
			flow: Right,
			align: { x: 0., y: 0.5 }
			margin: { top: 0., right: 5., bottom: 0., left: 0.}
			padding: 0.0,
			spacing: 7.5
			
			likes = <IconButton> {
				draw_icon: {svg_file: (ICO_FAV)} icon_walk: {width: 11.0, height: Fit}
				margin: 0, padding: 0.
			}
			comments = <IconButton> {
				draw_icon: {svg_file: (ICO_COMMENT)} icon_walk: {width: 11.0, height: Fit}, text: "7"
				margin: 0, padding: 0.
			}
			
			reply = <IconButton> {
				draw_icon: {svg_file: (ICO_REPLY)} icon_walk: {width: 12.0, height: Fit}, text: ""
				margin: 0, padding: 0.
			}
		}
	}
	
	Post = <View> {
		width: Fill, height: Fit,
		margin: { top: 0, right: 10., bottom: 10., left: 10. }
		flow: Down,

		padding: 0.0,
		spacing: 0.0
		
		body = <RoundedView> {
			width: Fill, height: Fit
			flow: Right,
			padding: 10.0,
			spacing: 10.0
			show_bg: true,
			draw_bg: {color: #fff}

			profile = <View> {
				width: Fit, height: Fit,
				margin: {top: 2.5}
				flow: Down,
				padding: 0.0
				profile_img = <Image> {
					source: (IMG_PROFILE_A)
					margin: 0,
					width: 40., height: 40.
					draw_bg: {
						fn pixel(self) -> vec4 {
							let sdf = Sdf2d::viewport(self.pos * self.rect_size);
							let c = self.rect_size * 0.5;
							sdf.circle(c.x, c.y, c.x - 2.)
							sdf.fill_keep(self.get_color());
							sdf.stroke((COLOR_PROFILE_CIRCLE), 1);
							return sdf.result
						}
					}
				}
			}
			content = <View> {
				width: Fill,
				height: Fit
				spacing: 10.,
				flow: Down,
				padding: 0.0
				
				<View> {
					flow: Right,
					height: Fit, width: Fill,
					spacing: 5.
					margin: { bottom: 3.0, top: 5.}
					align: { x: 0., y: 0. }

					meta = <Pbold> {
						width: Fit,
						margin: 0.,
						draw_text: {
							color: (COLOR_USER)
						}
						text: "Peter"
					}
					<P> {
						text: "13h",
						draw_text: {
							color: (COLOR_META)
						}
					}

					<PostMenu> {}
				}
				// <LineH> {
				//     margin: {top: 10.0, bottom: 5.0}
				// }

				text = <P> {
					width: Fill,
					height: Fit
					draw_text: {
						text_style: { line_spacing: 1.3 }
						wrap: Word,
						color: (COLOR_P)
					}
					text: ""
				}

				<TextInput> {
					width: Fill,

					empty_message: "Comment"

					draw_bg: {
						instance radius: (THEME_CORNER_RADIUS)
						instance hover: 0.0
						instance focus: 0.0
						instance bodytop: #00000008
						instance bodybottom: #00000010

						fn pixel(self) -> vec4 {
							let sdf = Sdf2d::viewport(self.pos * self.rect_size);
							let grad_top = 5.0;
							let grad_bot = 1.5;

							let body = mix(
								self.bodytop,
								self.bodybottom,
								self.focus
							);

							let body_transp = (THEME_COLOR_D_HIDDEN)

							let top_gradient = mix(
								body_transp,
								#00000030,
								max(0.0, grad_top - sdf.pos.y) / grad_top
							);

							let bot_gradient = mix(
								(THEME_COLOR_BEVEL_LIGHT),
								top_gradient,
								clamp((self.rect_size.y - grad_bot - sdf.pos.y - 1.0) / grad_bot, 0.0, 1.0)
							);

							sdf.box(
								1.,
								1.,
								self.rect_size.x - 2.0,
								self.rect_size.y - 2.0,
								self.radius
							)

							sdf.fill_keep(body)

							sdf.stroke(
								bot_gradient,
								THEME_BEVELING * 0.9
							)

							return sdf.result
						}
					}

					draw_text: {
						instance hover: 0.0
						instance focus: 0.0
						wrap: Word,
						text_style: <THEME_FONT_REGULAR> {
							line_spacing: (THEME_FONT_LINE_SPACING),
							font_size: (THEME_FONT_SIZE_P)
						}
						fn get_color(self) -> vec4 {
							return
							mix(
								mix(#00000099, #000000CC, self.hover),
								mix(#00000033, #00000055, self.hover),
								self.is_empty
							)
						}
					}

				}
			}
		}
		
	}
	
	PostImage = <View> {
		width: Fill, height: Fit
		flow: Down,
		padding: 0.0,
		spacing: 0.0
		
		hero = <Image> {
			source: (IMG_A),
			margin: 0,
			fit: Biggest,
			width: Fill,
			height: 200
		}
		
		post = <Post> {
			margin: {top: -30.0}
			body = {
				padding: { top: 10., right: 10., left: 10., bottom: 0. }
				content = {
					meta = {
						margin: {bottom: 30.0, top: 10.0}
						draw_text: {
							color: (COLOR_META_INV)
						}
					}
				}
			}
		}
	}
	
	NewsFeed ={{NewsFeed}}{
		list = <PortalList>{
			TopSpace = <View> {height: 80}
			Post = <CachedView>{<Post> {}}
			PostImage = <PostImage> {}
			BottomSpace = <View> {height: 100}
		}
	}
	
	App = {{App}} {
		ui: <Window> {
			
			window: {inner_size: vec2(428, 926)},
			show_bg: true
			draw_bg: {
				fn pixel(self) -> vec4 {
					return (COLOR_BG);
				}
			}
			body = {
				flow: Overlay,
				padding: 0.0
				spacing: 0,
				align: {
					x: 0.0,
					y: 0.0
				},
				
				news_feed = <NewsFeed>{}
				
				<View> {
					flow: Down
					<Header> {}
					<Filler> {}
					<Menu> {}
				}
			}
		}
	}
}

app_main!(App);

#[derive(Live, LiveHook, Widget)]
struct NewsFeed{ 
	#[deref] view:View
}

impl Widget for NewsFeed{
	fn draw_walk(&mut self, cx:&mut Cx2d, scope:&mut Scope, walk:Walk)->DrawStep{
		while let Some(item) =  self.view.draw_walk(cx, scope, walk).step(){
			if let Some(mut list) = item.as_portal_list().borrow_mut() {
				list.set_item_range(cx, 0, 1000);
				while let Some(item_id) = list.next_visible_item(cx) {
					let template = match item_id {
						0 => live_id!(TopSpace),
						x if x % 5 == 0 => live_id!(PostImage),
						_ => live_id!(Post)
					};
					let item = list.item(cx, item_id, template);
					let text = match item_id % 4 {
						1 => format!("At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. id: {}", item_id),
						2 => format!("How are you? Item id: {}", item_id),
						3 => format!("Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. id: {}", item_id),
						_ => format!("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. 4 id {}", item_id),
					};
					item.label(id!(content.text)).set_text(&text);
					item.button(id!(likes)).set_text(&format!("{}", item_id % 23));
					item.button(id!(comments)).set_text(&format!("{}", item_id % 6));
					item.draw_all(cx, &mut Scope::empty());
				}
			}
		}
		DrawStep::done()
	}
	fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope){
		self.view.handle_event(cx, event, scope)
	}
}

#[derive(Live, LiveHook)]
pub struct App {
	#[live] ui: WidgetRef,
}

impl LiveRegister for App {
	fn live_register(cx: &mut Cx) {
		crate::makepad_widgets::live_design(cx);
	} 
}

impl MatchEvent for App {
	fn handle_actions(&mut self, _cx:&mut Cx, actions:&Actions){
		let news_feeds = self.ui.portal_list_set(ids!(news_feed.list));
		for (item_id, item) in news_feeds.items_with_actions(&actions) {
			if item.button(id!(likes)).clicked(&actions) {
				log!("hello {}", item_id);
			}  
		}
	}
}

impl AppMain for App {
	fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
		self.match_event(cx, event);
		self.ui.handle_event(cx, event, &mut Scope::empty());
	}
}
