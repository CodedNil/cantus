self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.cantus;
  settingsFormat = pkgs.formats.toml { };
in
{
  options.programs.cantus = {
    enable = lib.mkEnableOption "cantus, a beautiful interactive music widget for wayland";

    package = lib.mkPackageOption pkgs "cantus" { };

    autoStart = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "If the cantus widget should be started automatically";
    };

    settings = lib.mkOption {
      type = lib.types.nullOr lib.types.attrs;
      default = null;
      description = "Settings written as TOML to `~/.config/cantus/cantus.toml";
      example = lib.literalExpression ''
        {
          monitor = "eDP-1;
          width = 1050.0;
          height = 40.0;
          timeline_future_minutes = 12.0;
          timeline_past_minutes = 1.5;
          history_width = 100.0;
          playlists = [ "Rock & Roll" "Instrumental" "Pop" ];
          ratings_enabled = true;
        }
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile = lib.optionalAttrs (cfg.settings != null) {
      "cantus/cantus.toml".source = settingsFormat.generate "cantus.toml" cfg.settings;
    };

    systemd.user.services.cantus = lib.mkIf cfg.autoStart {
      Unit = {
        Description = "A beautiful interactive music widget for wayland";
        After = [ config.wayland.systemd.target ];
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
