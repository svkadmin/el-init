// src/scripts.rs

use crate::{MenuNode, OsDistribution, ScriptCategory};
use std::{cell::RefCell, rc::Rc};

// The item macro now takes a category.
macro_rules! item {
    ($name:expr, $func:expr, $cat:expr) => {
        Rc::new(RefCell::new(MenuNode::Item {
            name: $name.to_string(),
            script_fn: $func,
            selected: false,
            category: $cat,
        }))
    };
}

// Helper macro to create a branch node (a sub-menu)
macro_rules! menu {
    ($name:expr, $($child:expr),*) => {
        Rc::new(RefCell::new(MenuNode::Menu {
            name: $name.to_string(),
            children: vec![$($child),*],
        }))
    };
}

/// Holds all scripts and dynamic names for a specific OS.
pub struct ScriptSet {
    // KVM
    kvm_base: fn() -> &'static str,
    kvm_full: fn() -> &'static str,
    kvm_virt_manager: fn() -> &'static str,
    kvm_tigervnc: fn() -> &'static str,
    kvm_remmina: fn() -> &'static str,
    kvm_libvirt_net_create: fn() -> &'static str,
    // Cockpit
    cockpit_base: fn() -> &'static str,
    cockpit_full: fn() -> &'static str,
    cockpit_storage: fn() -> &'static str,
    cockpit_podman: fn() -> &'static str,
    cockpit_files: fn() -> &'static str,
    cockpit_image_builder: fn() -> &'static str,
    cockpit_machines: fn() -> &'static str,
    // XEN
    install_xen: fn() -> &'static str,
    // Gnome
    gnome_base: fn() -> &'static str,
    gnome_full: fn() -> &'static str,
    // Gnome Extensions
    gnome_ext_forge: fn() -> &'static str,
    gnome_ext_tile: fn() -> &'static str,
    gnome_ext_paperwm: fn() -> &'static str,
    gnome_ext_hspacing: fn() -> &'static str,
    gnome_ext_vitals: fn() -> &'static str,
    gnome_ext_just_perfection: fn() -> &'static str,
    gnome_ext_search_light: fn() -> &'static str,
    // Gnome Apps
    app_ptyxis: fn() -> &'static str,
    app_konsole: fn() -> &'static str,
    app_alacritty: fn() -> &'static str,
    app_ghostty: fn() -> &'static str,
    app_filezilla: fn() -> &'static str,
    app_remmina: fn() -> &'static str,
    app_firefox: fn() -> &'static str,
    app_chromium: fn() -> &'static str,
    // Sway
    sway_compile_1_10: fn() -> &'static str,
    sway_wofi: fn() -> &'static str,
    sway_swaybg: fn() -> &'static str,
    sway_waybar: fn() -> &'static str,
    // Repositories
    repo_rt: fn() -> &'static str,
    repo_plus: fn() -> &'static str,
    repo_nfv: fn() -> &'static str,
    repo_ha: fn() -> &'static str,
    repo_extras: fn() -> &'static str,
    repo_devel: fn() -> &'static str,
    repo_crb: fn() -> &'static str,
    repo_baseos: fn() -> &'static str,
    repo_appstream: fn() -> &'static str,
    repo_epel: fn() -> &'static str,
    repo_flathub: fn() -> &'static str,
    // FIX: Add Networking fields
    net_vpn_ovpn: fn() -> &'static str,
    net_vpn_l2tp: fn() -> &'static str,
    net_vpn_sswan: fn() -> &'static str,
    net_vpn_lswan: fn() -> &'static str,
    net_vpn_pptp: fn() -> &'static str,
    net_vpn_oconn: fn() -> &'static str,
}

/// This function is the single source of truth for OS-specific scripts.
pub fn get_script_set(_os: OsDistribution) -> ScriptSet {
    ScriptSet {
        // KVM
        kvm_base: scripts_virt::kvm_base,
        kvm_full: scripts_virt::kvm_full,
        kvm_virt_manager: scripts_virt::kvm_virt_manager,
        kvm_tigervnc: scripts_virt::kvm_tigervnc,
        kvm_remmina: scripts_virt::kvm_remmina,
        kvm_libvirt_net_create: scripts_virt::kvm_libvirt_net_create,
        // Cockpit
        cockpit_base: scripts_virt::cockpit_base,
        cockpit_full: scripts_virt::cockpit_full,
        cockpit_storage: scripts_virt::cockpit_storage,
        cockpit_podman: scripts_virt::cockpit_podman,
        cockpit_files: scripts_virt::cockpit_files,
        cockpit_image_builder: scripts_virt::cockpit_image_builder,
        cockpit_machines: scripts_virt::cockpit_machines,
        // XEN
        install_xen: scripts_virt::install_xen,
        // Gnome
        gnome_base: scripts_gnome::base_install,
        gnome_full: scripts_gnome::full_install,
        // Gnome Extensions
        gnome_ext_forge: scripts_gnome_ext::placeholder,
        gnome_ext_tile: scripts_gnome_ext::placeholder,
        gnome_ext_paperwm: scripts_gnome_ext::placeholder,
        gnome_ext_hspacing: scripts_gnome_ext::placeholder,
        gnome_ext_vitals: scripts_gnome_ext::placeholder,
        gnome_ext_just_perfection: scripts_gnome_ext::placeholder,
        gnome_ext_search_light: scripts_gnome_ext::placeholder,
        // Gnome Apps
        app_ptyxis: scripts_gnome_apps::placeholder,
        app_konsole: scripts_gnome_apps::konsole,
        app_alacritty: scripts_gnome_apps::placeholder,
        app_ghostty: scripts_gnome_apps::placeholder,
        app_filezilla: scripts_gnome_apps::filezilla,
        app_remmina: scripts_gnome_apps::remmina,
        app_firefox: scripts_gnome_apps::firefox,
        app_chromium: scripts_gnome_apps::chromium,
        // Sway
        sway_compile_1_10: scripts_sway::compile_from_source,
        sway_wofi: scripts_sway::install_wofi,
        sway_swaybg: scripts_sway::install_swaybg,
        sway_waybar: scripts_sway::install_waybar,
        // Repositories (Rocky Specific)
        repo_rt: scripts_repos::add_rt,
        repo_plus: scripts_repos::add_plus,
        repo_nfv: scripts_repos::add_nfv,
        repo_ha: scripts_repos::add_ha,
        repo_extras: scripts_repos::add_extras,
        repo_devel: scripts_repos::add_devel,
        repo_crb: scripts_repos::add_crb,
        repo_baseos: scripts_repos::add_baseos,
        repo_appstream: scripts_repos::add_appstream,
        repo_epel: scripts_repos::add_epel,
        repo_flathub: scripts_repos::add_flathub,
        // FIX: Populate Networking fields
        net_vpn_ovpn: scripts_net::install_vpn_ovpn,
        net_vpn_l2tp: scripts_net::install_vpn_l2tp,
        net_vpn_sswan: scripts_net::install_vpn_sswan,
        net_vpn_lswan: scripts_net::install_vpn_lswan,
        net_vpn_pptp: scripts_net::install_vpn_pptp,
        net_vpn_oconn: scripts_net::install_vpn_oconn,
    }
}

/// Recursively sorts the children of menu nodes alphabetically.
fn sort_menu_recursively(node: &Rc<RefCell<MenuNode>>) {
    if let Ok(mut node_borrow) = node.try_borrow_mut() {
        if let MenuNode::Menu { children, .. } = &mut *node_borrow {
            children.sort_by(|a, b| {
                let a_name = match &*a.borrow() {
                    MenuNode::Menu { name, .. } => name.clone(),
                    MenuNode::Item { name, .. } => name.clone(),
                };
                let b_name = match &*b.borrow() {
                    MenuNode::Menu { name, .. } => name.clone(),
                    MenuNode::Item { name, .. } => name.clone(),
                };
                a_name.to_lowercase().cmp(&b_name.to_lowercase())
            });

            for child in children {
                sort_menu_recursively(child);
            }
        }
    }
}

/// Builds the menu tree using a generic ScriptSet.
pub fn build_menu_tree(os: OsDistribution) -> Rc<RefCell<MenuNode>> {
    let scripts = get_script_set(os);

    let main_menu = menu!("Main Menu",
        menu!("Virtualization",
            menu!("Virtualization Engines",
                menu!("KVM Core & Tools",
                    item!("Base Installation", scripts.kvm_base, ScriptCategory::General),
                    item!("Full Installation", scripts.kvm_full, ScriptCategory::General),
                    menu!("Modules",
                        item!("virt-manager", scripts.kvm_virt_manager, ScriptCategory::General),
                        item!("tigervnc", scripts.kvm_tigervnc, ScriptCategory::General),
                        item!("remmina", scripts.kvm_remmina, ScriptCategory::General)
                    ),
                    menu!("Setup Scripts",
                        item!("libvirt network create", scripts.kvm_libvirt_net_create, ScriptCategory::General)
                    )
                ),
                menu!("XEN Core & Tools",
                    item!("Base Installation", scripts.install_xen, ScriptCategory::General)
                ),
                menu!("XEN Management",)
            ),
            menu!("KVM Management",
                menu!("Cockpit",
                    item!("Base Installation", scripts.cockpit_base, ScriptCategory::General),
                    item!("Full Installation", scripts.cockpit_full, ScriptCategory::General),
                    menu!("Modules",
                        item!("storage", scripts.cockpit_storage, ScriptCategory::General),
                        item!("podman", scripts.cockpit_podman, ScriptCategory::General),
                        item!("files", scripts.cockpit_files, ScriptCategory::General),
                        item!("image builder", scripts.cockpit_image_builder, ScriptCategory::General),
                        item!("machines", scripts.cockpit_machines, ScriptCategory::General)
                    )
                )
            )
        ),
        menu!("Graphical Environments",
            menu!("Gnome DE - STABLE",
                menu!("Environment Installation",
                    item!("Base Installation", scripts.gnome_base, ScriptCategory::General),
                    item!("Full Installation", scripts.gnome_full, ScriptCategory::General)
                ),
                menu!("Customization / Extensions",
                    menu!("Tiling WM",
                        item!("Forge", scripts.gnome_ext_forge, ScriptCategory::General),
                        item!("Tile", scripts.gnome_ext_tile, ScriptCategory::General),
                        item!("PaperWM", scripts.gnome_ext_paperwm, ScriptCategory::General)
                    ),
                    menu!("Top Bar",
                        item!("status area horizontal spacing", scripts.gnome_ext_hspacing, ScriptCategory::General),
                        item!("vitals", scripts.gnome_ext_vitals, ScriptCategory::General)
                    ),
                    menu!("Tweaks",
                        item!("Just Perfection", scripts.gnome_ext_just_perfection, ScriptCategory::General)
                    ),
                    menu!("Search / Launchers",
                        item!("Search Light", scripts.gnome_ext_search_light, ScriptCategory::General)
                    )
                ),
                menu!("Applications / Packages",
                    menu!("Terminals",
                        item!("Ptyxis", scripts.app_ptyxis, ScriptCategory::General),
                        item!("Konsole", scripts.app_konsole, ScriptCategory::General),
                        item!("Allacritty", scripts.app_alacritty, ScriptCategory::General),
                        item!("Ghostty", scripts.app_ghostty, ScriptCategory::General)
                    ),
                    menu!("Remote Connection",
                        item!("Filezilla", scripts.app_filezilla, ScriptCategory::General),
                        item!("Remmina", scripts.app_remmina, ScriptCategory::General)
                    ),
                    menu!("Browsers",
                        item!("Firefox", scripts.app_firefox, ScriptCategory::General),
                        item!("Chromium", scripts.app_chromium, ScriptCategory::General)
                    )
                )
            ),
            menu!("Sway WM",
                menu!("Environment Installation",
                    menu!("Compile from Source",
                        item!("v1.10", scripts.sway_compile_1_10, ScriptCategory::General)
                    )
                ),
                menu!("Customization / Extentsions",
                    item!("Wofi", scripts.sway_wofi, ScriptCategory::General),
                    item!("Swaybg", scripts.sway_swaybg, ScriptCategory::General),
                    item!("Waybar", scripts.sway_waybar, ScriptCategory::General)
                )
            )
        ),
        // FIX: Add Networking menu back
        menu!("Networking",
            menu!("NetworkManager",
                item!("OpenVPN", scripts.net_vpn_ovpn, ScriptCategory::General),
                item!("OpenConnect", scripts.net_vpn_oconn, ScriptCategory::General),
                item!("L2TP", scripts.net_vpn_l2tp, ScriptCategory::General),
                item!("LibreSwan", scripts.net_vpn_lswan, ScriptCategory::General),
                item!("StrongSwan", scripts.net_vpn_sswan, ScriptCategory::General),
                item!("PPTP", scripts.net_vpn_pptp, ScriptCategory::General)
            )
        ),
        menu!("Repositories",
            menu!("Add Repositories (ROCKY LINUX SPECIFIC)",
                item!("realtime", scripts.repo_rt, ScriptCategory::Repository),
                item!("plus", scripts.repo_plus, ScriptCategory::Repository),
                item!("nfv", scripts.repo_nfv, ScriptCategory::Repository),
                item!("High availibility", scripts.repo_ha, ScriptCategory::Repository),
                item!("extras", scripts.repo_extras, ScriptCategory::Repository),
                item!("devel (WARNING)", scripts.repo_devel, ScriptCategory::Repository),
                item!("CRB (code ready builder)", scripts.repo_crb, ScriptCategory::Repository),
                item!("base OS", scripts.repo_baseos, ScriptCategory::Repository),
                item!("appstream", scripts.repo_appstream, ScriptCategory::Repository),
                item!("epel", scripts.repo_epel, ScriptCategory::Repository),
                item!("flathub", scripts.repo_flathub, ScriptCategory::Repository)
            )
        )
    );

    sort_menu_recursively(&main_menu);
    main_menu
}

// --- Script Content Modules ---

mod scripts_virt {
    pub fn kvm_base() -> &'static str { "sudo dnf install -y qemu-kvm libvirt-daemon-config-network libvirt-daemon-kvm" }
    pub fn kvm_full() -> &'static str { "sudo dnf install -y @virtualization virt-top libguestfs-tools" }
    pub fn kvm_virt_manager() -> &'static str { "sudo dnf install -y virt-manager" }
    pub fn kvm_tigervnc() -> &'static str { "sudo dnf install -y tigervnc-server" }
    pub fn kvm_remmina() -> &'static str { "sudo dnf install -y remmina" }
    pub fn kvm_libvirt_net_create() -> &'static str { "echo 'Placeholder for libvirt network creation script'" }
    
    pub fn cockpit_base() -> &'static str { "sudo dnf install -y cockpit\nsudo systemctl enable --now cockpit.socket" }
    pub fn cockpit_full() -> &'static str { "sudo dnf install -y cockpit cockpit-machines cockpit-podman cockpit-storaged\nsudo systemctl enable --now cockpit.socket" }
    pub fn cockpit_storage() -> &'static str { "sudo dnf install -y cockpit-storaged" }
    pub fn cockpit_podman() -> &'static str { "sudo dnf install -y cockpit-podman" }
    pub fn cockpit_files() -> &'static str { "echo 'cockpit-files is part of the core cockpit package'" }
    pub fn cockpit_image_builder() -> &'static str { "sudo dnf install -y cockpit-composer" }
    pub fn cockpit_machines() -> &'static str { "sudo dnf install -y cockpit-machines" }
    
    pub fn install_xen() -> &'static str { "sudo dnf install -y xen\nsudo systemctl enable xen-qemu-dom0-disk-backend.service" }
}

mod scripts_gnome {
    pub fn base_install() -> &'static str { "sudo dnf install -y gdm gnome-shell gnome-terminal" }
    pub fn full_install() -> &'static str { "sudo dnf groupinstall -y 'Workstation'" }
}

mod scripts_gnome_ext {
    pub fn placeholder() -> &'static str { "echo 'GNOME Shell extension installation must be done manually or via a dedicated script.'" }
}

mod scripts_gnome_apps {
    pub fn konsole() -> &'static str { "sudo dnf install -y konsole" }
    pub fn filezilla() -> &'static str { "sudo dnf install -y filezilla" }
    pub fn remmina() -> &'static str { "sudo dnf install -y remmina" }
    pub fn firefox() -> &'static str { "sudo dnf install -y firefox" }
    pub fn chromium() -> &'static str { "sudo dnf install -y chromium" }
    pub fn placeholder() -> &'static str { "echo 'This app is not in the default repos or requires special installation.'" }
}

mod scripts_sway {
    pub fn compile_from_source() -> &'static str { "echo 'Placeholder for Sway v1.10 compilation script'" }
    pub fn install_wofi() -> &'static str { "sudo dnf install -y wofi" }
    pub fn install_swaybg() -> &'static str { "sudo dnf install -y swaybg" }
    pub fn install_waybar() -> &'static str { "sudo dnf install -y waybar" }
}

mod scripts_repos {
    pub fn add_rt() -> &'static str { "sudo dnf config-manager --set-enabled rt" }
    pub fn add_plus() -> &'static str { "sudo dnf config-manager --set-enabled plus" }
    pub fn add_nfv() -> &'static str { "sudo dnf config-manager --set-enabled nfv" }
    pub fn add_ha() -> &'static str { "sudo dnf config-manager --set-enabled ha" }
    pub fn add_extras() -> &'static str { "sudo dnf config-manager --set-enabled extras" }
    pub fn add_devel() -> &'static str { "sudo dnf config-manager --set-enabled devel" }
    pub fn add_crb() -> &'static str { "sudo dnf config-manager --set-enabled crb" }
    pub fn add_baseos() -> &'static str { "sudo dnf config-manager --set-enabled baseos" }
    pub fn add_appstream() -> &'static str { "sudo dnf config-manager --set-enabled appstream" }
    pub fn add_epel() -> &'static str { "sudo dnf config-manager --set-enabled crb\nsudo dnf install -y 'https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm'" }
    pub fn add_flathub() -> &'static str { "sudo dnf install -y flatpak\nsudo flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo" }
}

mod scripts_net {
    pub fn install_vpn_ovpn() -> &'static str { "sudo dnf install -y NetworkManager-openvpn-gnome" }
    pub fn install_vpn_l2tp() -> &'static str { "sudo dnf install -y NetworkManager-l2tp-gnome" }
    pub fn install_vpn_sswan() -> &'static str { "sudo dnf install -y NetworkManager-strongswan-gnome" }
    pub fn install_vpn_lswan() -> &'static str { "sudo dnf install -y NetworkManager-libreswan-gnome" }
    pub fn install_vpn_pptp() -> &'static str { "sudo dnf install -y NetworkManager-pptp-gnome" }
    pub fn install_vpn_oconn() -> &'static str { "sudo dnf install -y NetworkManager-openconnect-gnome" }
}
