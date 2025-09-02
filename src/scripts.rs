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
    // Repositories
    crb_name: &'static str,
    add_crb: fn() -> &'static str,
    epel_name: &'static str,
    add_epel: fn() -> &'static str,
    add_ha: fn() -> &'static str,

    // Virtualization
    install_kvm: fn() -> &'static str,
    install_xen: fn() -> &'static str,
}

/// This function is the single source of truth for OS-specific scripts.
/// To add a new OS or change a script, you only need to modify this function.
pub fn get_script_set(os: OsDistribution) -> ScriptSet {
    // Define pointers to common scripts shared by community distros
    let community_crb = ("CRB", scripts_repos::add_crb_community as fn() -> &'static str);
    let community_epel = ("EPEL (with CRB)", scripts_repos::add_epel_community as fn() -> &'static str);
    let community_ha = scripts_repos::add_ha_community as fn() -> &'static str;
    let community_kvm = scripts_virt::install_kvm_community as fn() -> &'static str;

    match os {
        OsDistribution::Rhel => ScriptSet {
            crb_name: "CodeReady Builder",
            add_crb: scripts_repos::add_crb_rhel,
            epel_name: "EPEL (with CodeReady)",
            add_epel: scripts_repos::add_epel_rhel,
            add_ha: scripts_repos::add_ha_rhel_centos,
            install_kvm: scripts_virt::install_kvm_rhel,
            install_xen: scripts_virt::install_xen_rhel,
        },
        // Rocky, AlmaLinux, CentOS, and Unknown all use the same community scripts
        _ => ScriptSet {
            crb_name: community_crb.0,
            add_crb: community_crb.1,
            epel_name: community_epel.0,
            add_epel: community_epel.1,
            add_ha: community_ha,
            install_kvm: community_kvm,
            install_xen: scripts_virt::install_xen_community,
        },
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
    // Get the appropriate set of scripts for the detected OS
    let scripts = get_script_set(os);

    let main_menu = menu!("Main Menu",
        menu!("Virtualization",
            menu!("Virtualization Engines",
                item!("KVM (Core & Tools)", scripts.install_kvm, ScriptCategory::General),
                item!("XEN (Core & Tools)", scripts.install_xen, ScriptCategory::General)
            ),
            menu!("Cockpit",
                item!("Minimal Install", scripts_virt::install_cockpit_minimal, ScriptCategory::General),
                item!("Full Install (with Machines)", scripts_virt::install_cockpit_full, ScriptCategory::General)
            )
        ),
        menu!("Graphical Environments",
            menu!("Gnome DE",
                menu!("Environment Installation",
                    item!("Minimal Installation", scripts_gnome::minimal_install, ScriptCategory::General),
                    item!("Full Installation", scripts_gnome::full_install, ScriptCategory::General)
                )
            ),
            menu!("Sway WM",
                menu!("Environment Installation",
                    item!("Compile from Source", scripts_sway::compile_from_source, ScriptCategory::General)
                ),
                menu!("Customization",
                    item!("Wofi", scripts_sway::install_wofi, ScriptCategory::General)
                )
            )
        ),
        menu!("Repositories",
            menu!("Add Repositories",
                item!("CEPH", scripts_repos::add_ceph, ScriptCategory::Repository),
                item!(scripts.crb_name, scripts.add_crb, ScriptCategory::Repository),
                item!(scripts.epel_name, scripts.add_epel, ScriptCategory::Repository),
                item!("Flathub", scripts_repos::add_flathub, ScriptCategory::Repository),
                item!("Real-Time (RT)", scripts_repos::add_rt, ScriptCategory::Repository),
                item!("High Availability (HA)", scripts.add_ha, ScriptCategory::Repository)
            )
        ),
        menu!("Networking",
            menu!("NetworkManager",
                item!("OpenVPN", scripts_net::install_vpn_ovpn, ScriptCategory::General),
                item!("OpenConnect", scripts_net::install_vpn_oconn, ScriptCategory::General),
                item!("L2TP", scripts_net::install_vpn_l2tp, ScriptCategory::General),
                item!("LibreSwan", scripts_net::install_vpn_lswan, ScriptCategory::General),
                item!("StrongSwan", scripts_net::install_vpn_sswan, ScriptCategory::General),
                item!("PPTP", scripts_net::install_vpn_pptp, ScriptCategory::General)
            )
        ),
        menu!("Hardening",),
        menu!("Kernels",),
        menu!("CPU Optimizations",),
        menu!("GPU",),
        menu!("Monitoring",)
    );

    sort_menu_recursively(&main_menu);
    main_menu
}

// --- Script Content Modules ---
// These modules now contain the raw script strings, organized by OS where needed.

mod scripts_gnome {
    pub fn minimal_install() -> &'static str { "sudo dnf install -y gdm gnome-browser-connector\nsudo systemctl set-default graphical.target" }
    pub fn full_install() -> &'static str { "sudo dnf groupinstall -y 'Workstation'\nsudo systemctl set-default graphical.target" }
}

mod scripts_sway {
    pub fn compile_from_source() -> &'static str { "sudo dnf install -y ninja-build meson gcc wayland-devel wayland-protocols-devel libinput-devel libxcb-devel libxkbcommon-devel pixman-devel" }
    pub fn install_wofi() -> &'static str { "sudo dnf install -y wofi" }
}

mod scripts_repos {
    // Shared
    pub fn add_ceph() -> &'static str { "sudo dnf install -y ceph-common" }
    pub fn add_flathub() -> &'static str { "flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo" }
    pub fn add_rt() -> &'static str { "sudo dnf config-manager --set-enabled rt" }

    // RHEL-specific
    pub fn add_crb_rhel() -> &'static str { "sudo subscription-manager repos --enable codeready-builder-for-rhel-9-$(arch)-rpms" }
    pub fn add_epel_rhel() -> &'static str { "sudo subscription-manager repos --enable codeready-builder-for-rhel-9-$(arch)-rpms\nsudo dnf install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-9.noarch.rpm" }
    
    // Community Distros (Rocky, Alma, CentOS)
    pub fn add_crb_community() -> &'static str { "sudo dnf config-manager --set-enabled crb" }
    pub fn add_epel_community() -> &'static str { "sudo dnf config-manager --set-enabled crb\nsudo dnf install -y epel-release" }
    
    // Mixed
    pub fn add_ha_rhel_centos() -> &'static str { "sudo dnf config-manager --set-enabled ha" }
    pub fn add_ha_community() -> &'static str { "echo '# For Rocky/Alma, HA packages are in the \\'highavailability\\' repo.'\necho '# Enable with: sudo dnf config-manager --set-enabled highavailability'" }
}

mod scripts_virt {
    // Shared (but defined separately for clarity in the struct)
    pub fn install_kvm_rhel() -> &'static str { "sudo dnf install -y @virtualization\nsudo systemctl enable --now libvirtd" }
    pub fn install_kvm_community() -> &'static str { "sudo dnf install -y @virtualization\nsudo systemctl enable --now libvirtd" }
    pub fn install_xen_rhel() -> &'static str { "sudo dnf install -y xen\nsudo systemctl enable xen-qemu-dom0-disk-backend.service" }
    pub fn install_xen_community() -> &'static str { "sudo dnf install -y xen\nsudo systemctl enable xen-qemu-dom0-disk-backend.service" }
    
    // Truly shared
    pub fn install_cockpit_minimal() -> &'static str { "sudo dnf install -y cockpit\nsudo systemctl enable --now cockpit.socket\nsudo firewall-cmd --add-service=cockpit --permanent\nsudo firewall-cmd --reload" }
    pub fn install_cockpit_full() -> &'static str { "sudo dnf install -y cockpit cockpit-machines cockpit-podman\nsudo systemctl enable --now cockpit.socket\nsudo firewall-cmd --add-service=cockpit --permanent\nsudo firewall-cmd --reload" }
}

mod scripts_net {
    pub fn install_vpn_ovpn() -> &'static str { "sudo dnf install -y NetworkManager-openvpn-gnome" }
    pub fn install_vpn_l2tp() -> &'static str { "sudo dnf install -y NetworkManager-l2tp-gnome" }
    pub fn install_vpn_sswan() -> &'static str { "sudo dnf install -y NetworkManager-strongswan-gnome" }
    pub fn install_vpn_lswan() -> &'static str { "sudo dnf install -y NetworkManager-libreswan-gnome" }
    pub fn install_vpn_pptp() -> &'static str { "sudo dnf install -y NetworkManager-pptp-gnome" }
    pub fn install_vpn_oconn() -> &'static str { "sudo dnf install -y NetworkManager-openconnect-gnome" }
}
