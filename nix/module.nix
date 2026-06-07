{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib)
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    optional
    optionalAttrs
    types
    ;

  cfg = config.programs.cantus;
  settingsFormat = pkgs.formats.toml { };
  number = types.either types.int types.float;
  settingsType = types.submodule {
    options = {
      spotify_client_id = mkOption {
        type = types.nullOr types.str;
        default = null;
        description = "Spotify client ID to use for authentication.";
      };

      monitor = mkOption {
        type = types.nullOr types.str;
        default = null;
        description = "Monitor to display Cantus on.";
      };

      width = mkOption {
        type = number;
        default = 1050.0;
        description = "Width of the timeline in pixels.";
      };

      height = mkOption {
        type = number;
        default = 50.0;
        description = "Height of the timeline in pixels.";
      };

      layer = mkOption {
        type = types.enum [
          "background"
          "bottom"
          "top"
          "overlay"
        ];
        default = "top";
        description = "Layer shell layer to display Cantus on.";
      };

      layer_anchor = mkOption {
        type = types.enum [
          "top"
          "bottom"
        ];
        default = "top";
        description = "Screen edge Cantus should anchor to.";
      };

      timeline_future_minutes = mkOption {
        type = number;
        default = 12.0;
        description = "Minutes in the future to display in the timeline.";
      };

      timeline_past_minutes = mkOption {
        type = number;
        default = 1.5;
        description = "Minutes before the current time to display in the timeline.";
      };

      history_width = mkOption {
        type = number;
        default = 100.0;
        description = "Width in pixels where previous tracks are displayed.";
      };

      playlists = mkOption {
        type = types.listOf types.str;
        default = [ ];
        description = "Favourite playlists to display as buttons.";
      };

      ratings_enabled = mkOption {
        type = types.bool;
        default = false;
        description = "Whether star ratings should be enabled.";
      };
    };
  };
  tomlSettings = lib.filterAttrs (_: value: value != null) cfg.settings;
in
{
  options.programs.cantus = {
    enable = mkEnableOption "cantus, a beautiful interactive music widget for wayland";

    package = mkPackageOption pkgs "cantus" { };

    autoStart = mkOption {
      type = types.bool;
      default = true;
      description = "If the cantus widget should be started automatically";
    };

    settings = mkOption {
      type = types.nullOr settingsType;
      default = null;
      description = "Settings written as TOML to `~/.config/cantus/cantus.toml`.";
      example = {
        monitor = "eDP-1";
        width = 1050.0;
        height = 40.0;
        timeline_future_minutes = 12.0;
        timeline_past_minutes = 1.5;
        history_width = 100.0;
        playlists = [
          "Rock & Roll"
          "Instrumental"
          "Pop"
        ];
        ratings_enabled = true;
      };
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile = optionalAttrs (cfg.settings != null) {
      "cantus/cantus.toml".source = settingsFormat.generate "cantus.toml" tomlSettings;
    };

    systemd.user.services.cantus = mkIf cfg.autoStart {
      Unit = {
        Description = "A beautiful interactive music widget for wayland";
        After = [ config.wayland.systemd.target ];
        X-Restart-Triggers = optional (
          cfg.settings != null
        ) config.xdg.configFile."cantus/cantus.toml".source;
      };
      Service = {
        Type = "simple";
        ExecStart = "${cfg.package}/bin/cantus";
        Restart = "on-failure";
      };
      Install.WantedBy = [ config.wayland.systemd.target ];
    };
  };
}
