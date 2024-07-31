use iced::widget::{component, container, scrollable, Component};
use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use std::{cell::RefCell, rc::Rc};

use crate::i18n::{Translate, Translated};
use crate::{fl, i18n, Element, INPUT_PADDING};
use cofd::{
	character::InfoTrait,
	prelude::*,
	splat::{Splat, SplatTrait, XSplat, YSplat, ZSplat},
};
use iced::overlay::menu;

#[derive(Debug, Clone)]
pub struct InfoBar;

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Message {
	InfoTraitChanged(String, InfoTrait),
	XSplatChanged(XSplat),
	YSplatChanged(YSplat),
	ZSplatChanged(ZSplat),
}

impl InfoBar {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		match message {
			Message::InfoTraitChanged(val, _trait) => *character.info.get_mut(_trait) = val,
			Message::XSplatChanged(xsplat) => {
				if xsplat.name().eq("") {
					character.splat.set_xsplat(None);
				} else {
					character.splat.set_xsplat(Some(xsplat));
				}
				character.calc_mod_map();
			}
			Message::YSplatChanged(ysplat) => {
				if ysplat.name().eq("") {
					character.splat.set_ysplat(None);
				} else {
					character.splat.set_ysplat(Some(ysplat));
				}
				//character.calc_mod_map();
			}
			Message::ZSplatChanged(zsplat) => {
				if zsplat.name().eq("") {
					character.splat.set_zsplat(None);
				} else {
					character.splat.set_zsplat(Some(zsplat));
				}
			}
		}
	}

	#[allow(
		clippy::similar_names,
		clippy::single_match_else,
		clippy::too_many_lines
	)]
	pub fn view(&self, character: &Character) -> Element<Message> {
		let col3: Element<Message> = match character.splat {
			Splat::Mortal(..) => self.mk_info_col(
				vec![InfoTrait::Age, InfoTrait::Faction, InfoTrait::GroupName],
				&character,
			),
			_ => {
				let mut xsplats = character.splat.xsplats();
				let mut ysplats = character.splat.ysplats();
				let mut zsplats = character.splat.zsplats();

				if let Some(xsplat) = character.splat.custom_xsplat(fl!("custom")) {
					xsplats.push(xsplat);
				}
				if let Some(ysplat) = character.splat.custom_ysplat(fl!("custom")) {
					ysplats.push(ysplat);
				}
				if let Some(zsplat) = character.splat.custom_zsplat(fl!("custom")) {
					zsplats.push(zsplat);
				}

				let xsplats: Vec<Translated<XSplat>> =
					xsplats.into_iter().map(Into::into).collect();
				let ysplats: Vec<Translated<YSplat>> =
					ysplats.into_iter().map(Into::into).collect();
				let zsplats: Vec<Translated<ZSplat>> =
					zsplats.into_iter().map(Into::into).collect();

				let xsplat = character.splat.xsplat();
				let ysplat = character.splat.ysplat();
				let zsplat = character.splat.zsplat();

				let xsplat: Element<Message> = if let Some(xsplat) = xsplat.clone()
					&& xsplat.is_custom()
				{
					text_input("", xsplat.name())
						.on_input({
							let xsplat = xsplat.clone();
							move |val| {
								let mut xsplat = xsplat.clone();
								*xsplat.name_mut().unwrap() = val;
								Message::XSplatChanged(xsplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						xsplats,
						xsplat.map(Into::<Translated<XSplat>>::into),
						|val| Message::XSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let ysplat: Element<Message> = if let Some(ysplat) = ysplat.clone()
					&& ysplat.is_custom()
				{
					text_input("", ysplat.name())
						.on_input({
							let ysplat = ysplat.clone();
							move |val| {
								let mut ysplat = ysplat.clone();
								*ysplat.name_mut().unwrap() = val;
								Message::YSplatChanged(ysplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						ysplats,
						ysplat.map(Into::<Translated<YSplat>>::into),
						|val| Message::YSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let zsplat: Element<Message> = if let Some(zsplat) = zsplat.clone()
					&& zsplat.is_custom()
				{
					text_input("", zsplat.name())
						.on_input({
							let zsplat = zsplat.clone();
							move |val| {
								let mut zsplat = zsplat.clone();
								*zsplat.name_mut().unwrap() = val;
								Message::ZSplatChanged(zsplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						zsplats,
						zsplat.map(Into::<Translated<ZSplat>>::into),
						|val| Message::ZSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let xsplat_name = character
					.splat
					.xsplat_name()
					.map(|k| i18n::LANGUAGE_LOADER.get(k))
					.unwrap_or_default();
				let ysplat_name = character
					.splat
					.ysplat_name()
					.map(|k| i18n::LANGUAGE_LOADER.get(k))
					.unwrap_or_default();
				let zsplat_name = character
					.splat
					.zsplat_name()
					.map(|k| i18n::LANGUAGE_LOADER.get(k))
					.unwrap_or_default();

				row![
					column![
						text(format!("{xsplat_name}:")),
						text(format!("{ysplat_name}:")),
						text(format!("{zsplat_name}:"))
					]
					.spacing(3),
					column![xsplat, ysplat, zsplat]
						.spacing(1)
						.width(Length::Fill)
				]
				.width(Length::Fill)
				.spacing(5)
				.into()
			}
		};

		row![
			self.mk_info_col(
				vec![InfoTrait::Name, InfoTrait::Player, InfoTrait::Chronicle],
				&character
			),
			self.mk_info_col(
				vec![
					InfoTrait::VirtueAnchor,
					InfoTrait::ViceAnchor,
					InfoTrait::Concept
				],
				&character
			),
			col3,
		]
		.width(Length::Fill)
		.spacing(10)
		.into()
	}

	fn mk_info_col(&self, info: Vec<InfoTrait>, character: &Character) -> Element<Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(3)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for _trait in info {
			let str = match _trait {
				InfoTrait::VirtueAnchor => character.splat.virtue_anchor().translated(),
				InfoTrait::ViceAnchor => character.splat.vice_anchor().translated(),
				_ => _trait.translated(),
			};

			col1 = col1.push(text(format!("{}:", str)));
			col2 = col2.push(
				text_input("", character.info.get(_trait))
					.on_input(move |val| Message::InfoTraitChanged(val, _trait))
					.padding(INPUT_PADDING),
			);
		}

		row![col1, col2].width(Length::Fill).spacing(5).into()
	}
}
