let
	nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.05";
	pkgs = import nixpkgs {
		config = {
			allowUnfree = true;		
		}; 
		overlays = [
			
		];
	};

	unstablepkgs = fetchTarball https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
	unstable = import unstablepkgs {
		config = {
			allowUnfree = true;
		};
		overlays = [
		
		];
	};
in


pkgs.mkShellNoCC {
	packages = with pkgs; [
		vscode
		rustup
		gcc
		pkg-config
		dbus.dev
		(pkgs.python3.withPackages (python-pkgs: with python-pkgs; [
      			# select Python packages here
      			bleak
    		]))
		#unstable.blender
		#godot_4
		#nixGL
	];
}
