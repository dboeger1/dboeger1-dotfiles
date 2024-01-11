Name:
Version:
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files.

License: None
#URL:
Source0:
#
#BuildRequires:
#Requires:

BuildArch: noarch

%global app_destination_dir /opt/%{name}
%global app_source_dir %{buildroot}/%{app_destination_dir}

%description


%prep
%setup


%build


%install
(
    cd tmux &&
    exec install -D -t %{app_source_dir}/tmux \
        .tmux.conf
)

(
    cd neovim &&
    exec install -D -t %{app_source_dir}/neovim \
        init.lua \
        lazy-lock.json
)

(
    cd neovim/lua &&
    exec install -D -t %{app_source_dir}/neovim/lua \
        .luarc.json
)

(
    cd neovim/lua/plugins &&
    exec install -D -t %{app_source_dir}/neovim/lua/plugins \
        plugin_netrw.lua
)

(
    cd neovim/lua/plugins/lazy &&
    exec install -D -t %{app_source_dir}/neovim/lua/plugins/lazy \
        plugin_cmp.lua \
        plugin_kanagawa.lua \
        plugin_lualine.lua \
        plugin_mason.lua \
        plugin_nightfox.lua \
        plugin_snippy.lua \
        plugin_telescope.lua \
        plugin_telescope_fzf_native.lua \
        plugin_treesitter.lua
)

(
    cd neovim/lua/settings &&
    exec install -D -t %{app_source_dir}/neovim/lua/settings \
        indentation.lua \
        information.lua \
        key_maps.lua \
        tabs.lua \
        themes.lua \
        windows.lua
)


%check


%files


%changelog