FROM mcr.microsoft.com/devcontainers/base:noble

RUN <<-'EOF' bash
	set -eu -o pipefail
	apt-get update
	apt-get install -y --no-install-recommends ca-certificates pkg-config libssl-dev
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sed 's#/proc/self/exe#\/bin\/sh#g' | sh -s -- -y --no-update-default-toolchain 1>/dev/null
	rm -rf /var/lib/apt/lists/*
EOF

RUN <<-'EOF' bash
	set -eu -o pipefail
	[ -f /etc/dpkg/dpkg.cfg.d/excludes ] && sed -i'' "s:^path-exclude=.*:#path-exclude=.*:" /etc/dpkg/dpkg.cfg.d/excludes
	apt-get update
	apt-get install -y --no-install-recommends git fzf
	[ -f /etc/dpkg/dpkg.cfg.d/excludes ] && sed -i'' "s:^#path-exclude=.*:path-exclude=.*:" /etc/dpkg/dpkg.cfg.d/excludes
	git clone --depth=1 https://github.com/romkatv/powerlevel10k.git $HOME/.oh-my-zsh/custom/themes/powerlevel10k
	git clone --depth=1 https://github.com/zsh-users/zsh-autosuggestions $HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions
	git clone --depth=1 https://github.com/zsh-users/zsh-completions ${ZSH_CUSTOM:=~/.oh-my-zsh/custom}/plugins/zsh-completions
	cp $HOME/.oh-my-zsh/custom/themes/powerlevel10k/config/p10k-lean.zsh $HOME/.p10k.zsh
	sed -i'' -e 's/^plugins.*)/plugins=(z fzf git zsh-autosuggestions zsh-completions)/' \
	-e '/^ZSH_THEME=.*/{
r /dev/stdin
d
}' $HOME/.zshrc <<< $'if [[ "$PAGER" == "head -n 10000 | cat" ]]; then\n  ZSH_THEME="robbyrussell"\nelse\n  ZSH_THEME="powerlevel10k/powerlevel10k"\nfi'
	echo '[[ ! -f "$HOME/.p10k.zsh" ]] || source "$HOME/.p10k.zsh"' >> $HOME/.zshrc
	# https://github.com/ohmyzsh/ohmyzsh/issues/7426
	echo 'export LANG=en_US.UTF-8' >> $HOME/.zshrc
EOF