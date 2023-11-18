{ pkgs ? import <nixpkgs> {} }:

let
  #nodePackages = import ./node-packages.nix { inherit pkgs; };
in
pkgs.mkShell {
  name = "my-env";
  buildInputs = [
    pkgs.rustup
    pkgs.nodejs
    pkgs.yarn
    #nodePackages.shell
  ];

  shellHook = ''
    # Set up the environment variables for the individual projects
    export API_DIR=$PWD/api
    export WEBSOCKET_DIR=$PWD/websocket
    export WEB_DIR=$PWD/web

    # Set up the Rust environment
    rustup toolchain install stable
    rustup default stable

    # Navigate to the API directory and set up the Rust environment
    cd $API_DIR
    cargo fetch

    # Navigate to the WebSocket directory and set up the Rust environment
    cd $WEBSOCKET_DIR
    cargo fetch

    # Navigate back to the project root
    cd $PWD
  '';
}
