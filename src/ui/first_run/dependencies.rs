use relm4::prelude::*;
use adw::prelude::*;

use anime_launcher_sdk::is_available;

use crate::*;

use super::main::FirstRunAppMsg;

pub struct DependenciesApp {
    show_arch: bool,
    show_debian: bool,
    show_fedora: bool,
    show_list: bool
}

#[derive(Debug, Clone)]
pub enum DependenciesAppMsg {
    Continue,
    Exit
}

#[relm4::component(async, pub)]
impl SimpleAsyncComponent for DependenciesApp {
    type Input = DependenciesAppMsg;
    type Output = FirstRunAppMsg;
    type Init = ();

    view! {
        adw::PreferencesPage {
            set_hexpand: true,

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Label {
                    set_label: &tr!("missing-dependencies-title"),
                    add_css_class: "title-1"
                },

                gtk::Label {
                    set_label: &tr!("missing-dependencies-message"),
    
                    set_justify: gtk::Justification::Center,
                    set_wrap: true,
                    set_margin_top: 32
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 16,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_arch,

                        gtk::Label {
                            set_label: "Arch (pacman)"
                        },

                        gtk::Entry {
                            set_text: "sudo pacman -Syu git",
                            set_editable: false
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_debian,

                        gtk::Label {
                            set_label: "Debian / Ubuntu (apt)"
                        },

                        gtk::Entry {
                            set_text: "sudo apt install git",
                            set_editable: false
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_fedora,

                        gtk::Label {
                            set_label: "Fedora (dnf)"
                        },

                        gtk::Entry {
                            set_text: "sudo dnf install git",
                            set_editable: false
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 16,

                        #[watch]
                        set_visible: model.show_list,

                        adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "git"
                            }
                        }
                    }
                }
            },

            add = &adw::PreferencesGroup {
                set_valign: gtk::Align::Center,
                set_vexpand: true,
    
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 8,

                    gtk::Button {
                        set_label: &tr!("check"),
                        set_css_classes: &["suggested-action", "pill"],

                        connect_clicked => DependenciesAppMsg::Continue
                    },

                    gtk::Button {
                        set_label: &tr!("exit"),
                        add_css_class: "pill",

                        connect_clicked => DependenciesAppMsg::Exit
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, _sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let distro = whatadistro::identify();

        let mut model = Self {
            show_arch: match &distro {
                Some(distro) => distro.is_similar("arch"),
                None => false
            },

            show_debian: match &distro {
                Some(distro) => distro.is_similar("debian"),
                None => false
            },

            show_fedora: match &distro {
                Some(distro) => distro.is_similar("fedora"),
                None => false
            },

            show_list: false
        };

        model.show_list = !model.show_arch && !model.show_debian && !model.show_fedora;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            #[allow(unused_must_use)]
            DependenciesAppMsg::Continue => {
                if !is_available("git") {
                    sender.output(Self::Output::Toast {
                        title: tr!("package-not-available", {
                            "package" = "git"
                        }),
                        description: None
                    });
                }

                else {
                    sender.output(Self::Output::ScrollToDefaultPaths);
                }
            }

            DependenciesAppMsg::Exit => relm4::main_application().quit()
        }
    }
}
