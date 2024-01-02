// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn sandcastle() -> UserThemeFamily {
    UserThemeFamily {
        name: "Sandcastle".into(),
        author: "Zed Industries".into(),
        themes: vec![UserTheme {
            name: "Sandcastle".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x3d4350ff).into()),
                    border_variant: Some(rgba(0x3d4350ff).into()),
                    border_focused: Some(rgba(0x223232ff).into()),
                    border_selected: Some(rgba(0x223232ff).into()),
                    border_transparent: Some(rgba(0x00000000).into()),
                    border_disabled: Some(rgba(0x393f4aff).into()),
                    elevated_surface_background: Some(rgba(0x2b3039ff).into()),
                    surface_background: Some(rgba(0x2b3039ff).into()),
                    background: Some(rgba(0x333944ff).into()),
                    panel_background: Some(rgba(0x2b3039ff).into()),
                    element_background: Some(rgba(0x2b3039ff).into()),
                    element_hover: Some(rgba(0x313741ff).into()),
                    element_active: Some(rgba(0x3d4350ff).into()),
                    element_selected: Some(rgba(0x3d4350ff).into()),
                    element_disabled: Some(rgba(0x2b3039ff).into()),
                    drop_target_background: Some(rgba(0xa6978280).into()),
                    ghost_element_background: Some(rgba(0x00000000).into()),
                    ghost_element_hover: Some(rgba(0x313741ff).into()),
                    ghost_element_active: Some(rgba(0x3d4350ff).into()),
                    ghost_element_selected: Some(rgba(0x3d4350ff).into()),
                    ghost_element_disabled: Some(rgba(0x2b3039ff).into()),
                    text: Some(rgba(0xfdf4c1ff).into()),
                    text_muted: Some(rgba(0xa69782ff).into()),
                    text_placeholder: Some(rgba(0x827568ff).into()),
                    text_disabled: Some(rgba(0x827568ff).into()),
                    text_accent: Some(rgba(0x528b8bff).into()),
                    icon: Some(rgba(0xfdf4c1ff).into()),
                    icon_muted: Some(rgba(0xa69782ff).into()),
                    icon_disabled: Some(rgba(0x827568ff).into()),
                    icon_placeholder: Some(rgba(0xa69782ff).into()),
                    icon_accent: Some(rgba(0x528b8bff).into()),
                    status_bar_background: Some(rgba(0x333944ff).into()),
                    title_bar_background: Some(rgba(0x333944ff).into()),
                    toolbar_background: Some(rgba(0x282c34ff).into()),
                    tab_bar_background: Some(rgba(0x2b3039ff).into()),
                    tab_inactive_background: Some(rgba(0x2b3039ff).into()),
                    tab_active_background: Some(rgba(0x282c34ff).into()),
                    scrollbar_thumb_background: Some(rgba(0xfdf4c14c).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0x313741ff).into()),
                    scrollbar_thumb_border: Some(rgba(0x313741ff).into()),
                    scrollbar_track_background: Some(rgba(0x282c34ff).into()),
                    scrollbar_track_border: Some(rgba(0x2a2f38ff).into()),
                    editor_foreground: Some(rgba(0xfdf4c1ff).into()),
                    editor_background: Some(rgba(0x282c34ff).into()),
                    editor_gutter_background: Some(rgba(0x282c34ff).into()),
                    editor_subheader_background: Some(rgba(0x2b3039ff).into()),
                    editor_active_line_background: Some(rgba(0x2b3039bf).into()),
                    editor_highlighted_line_background: Some(rgba(0x2b3039ff).into()),
                    editor_line_number: Some(rgba(0xfdf4c159).into()),
                    editor_active_line_number: Some(rgba(0xfdf4c1ff).into()),
                    editor_invisible: Some(rgba(0xa69782ff).into()),
                    editor_wrap_guide: Some(rgba(0xfdf4c10d).into()),
                    editor_active_wrap_guide: Some(rgba(0xfdf4c11a).into()),
                    editor_document_highlight_read_background: Some(rgba(0x528b8b1a).into()),
                    editor_document_highlight_write_background: Some(rgba(0x7c6f6466).into()),
                    terminal_background: Some(rgba(0x282c34ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x5e5753ff).into()),
                    terminal_ansi_bright_red: Some(rgba(0x57333dff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x414f4aff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0x4e3f22ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x2c4444ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0x523a18ff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x414f4aff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xfdf4c1ff).into()),
                    terminal_ansi_black: Some(rgba(0x282c34ff).into()),
                    terminal_ansi_red: Some(rgba(0xb4637aff).into()),
                    terminal_ansi_green: Some(rgba(0x83a598ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xa07e3bff).into()),
                    terminal_ansi_blue: Some(rgba(0x528b8bff).into()),
                    terminal_ansi_magenta: Some(rgba(0xa87323ff).into()),
                    terminal_ansi_cyan: Some(rgba(0x83a598ff).into()),
                    terminal_ansi_white: Some(rgba(0xfdf4c1ff).into()),
                    link_text_hover: Some(rgba(0x528b8bff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    conflict: Some(rgba(0xa07e3bff).into()),
                    created: Some(rgba(0x83a598ff).into()),
                    deleted: Some(rgba(0xb4637aff).into()),
                    error: Some(rgba(0xb4637aff).into()),
                    hidden: Some(rgba(0x827568ff).into()),
                    hint: Some(rgba(0x528b8bff).into()),
                    ignored: Some(rgba(0xa69782ff).into()),
                    info: Some(rgba(0x528b8bff).into()),
                    modified: Some(rgba(0xa07e3bff).into()),
                    predictive: Some(rgba(0x83a598ff).into()),
                    renamed: Some(rgba(0x528b8bff).into()),
                    success: Some(rgba(0x83a598ff).into()),
                    unreachable: Some(rgba(0xa69782ff).into()),
                    warning: Some(rgba(0xa07e3bff).into()),
                    ..Default::default()
                },
                player: Some(PlayerColors(vec![
                    PlayerColor {
                        cursor: rgba(0x528b8bff).into(),
                        background: rgba(0x528b8bff).into(),
                        selection: rgba(0x528b8b3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xa87323ff).into(),
                        background: rgba(0xa87323ff).into(),
                        selection: rgba(0xa873233d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xa07e3bff).into(),
                        background: rgba(0xa07e3bff).into(),
                        selection: rgba(0xa07e3b3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xd75f5fff).into(),
                        background: rgba(0xd75f5fff).into(),
                        selection: rgba(0xd75f5f3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x83a598ff).into(),
                        background: rgba(0x83a598ff).into(),
                        selection: rgba(0x83a5983d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xb4637aff).into(),
                        background: rgba(0xb4637aff).into(),
                        selection: rgba(0xb4637a3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xa07e3bff).into(),
                        background: rgba(0xa07e3bff).into(),
                        selection: rgba(0xa07e3b3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x83a598ff).into(),
                        background: rgba(0x83a598ff).into(),
                        selection: rgba(0x83a5983d).into(),
                    },
                ])),
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x83a598ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa89984ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa89984ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x83a598ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "embedded".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfdf4c1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis.strong".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "enum".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "hint".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x727d68ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x83a598ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x83a598ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "predictive".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5c6152ff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "preproc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfdf4c1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "primary".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfdf4c1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd5c5a1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd5c5a1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd5c5a1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd5c5a1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd5c5a1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.escape".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa89984ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.regex".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special.symbol".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "text.literal".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xa07e3bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfdf4c1ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x83a598ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfdf4c1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x528b8bff).into()),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
