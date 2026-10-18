self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.weather-station-wrapper;
  stateDir = "/var/lib/weather-station-wrapper";
  configFile = "${stateDir}/config.json";
in
{
  options.services.weather-station-wrapper = {
    enable = lib.mkEnableOption "weather station wrapper";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "weather-station-wrapper.packages.\${system}.default";
    };

    port = lib.mkOption {
      type = lib.types.port;
      default = 6001;
    };

    openFirewall = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Open {option}`port` in the firewall.";
    };

    datasourceId = lib.mkOption {
      type = lib.types.str;
      default = "7a224a14-6e07-45a3-91da-b7584a5731c1";
    };

    baseUrl = lib.mkOption {
      type = lib.types.str;
      default = "https://rt2.ambientweather.net/";
      description = "Ambient Weather realtime API base URL.";
    };

    apiKeyFile = lib.mkOption {
      type = lib.types.path;
      description = "File containing the Ambient Weather API key (e.g. an agenix/sops secret).";
    };

    applicationKeyFile = lib.mkOption {
      type = lib.types.path;
      description = "File containing the Ambient Weather application key.";
    };

    logLevel = lib.mkOption {
      type = lib.types.str;
      default = "info";
      description = "Value of `RUST_LOG`.";
    };
  };

  config = lib.mkIf cfg.enable {
    networking.firewall.allowedTCPPorts = lib.mkIf cfg.openFirewall [ cfg.port ];

    systemd.services.weather-station-wrapper = {
      description = "Weather station wrapper";
      wantedBy = [ "multi-user.target" ];
      wants = [ "network-online.target" ];
      after = [ "network-online.target" ];

      environment.RUST_LOG = cfg.logLevel;

      # The service rewrites its config at runtime (POST /config, POST /activate),
      # so it lives in the state dir. Nix-managed fields are merged over it on
      # every start; runtime-only fields such as `scripting_api` survive.
      preStart = ''
        [ -s ${configFile} ] || echo '{}' > ${configFile}
        ${lib.getExe pkgs.jq} -n \
          --slurpfile existing ${configFile} \
          --argjson port ${toString cfg.port} \
          --arg datasource_id ${lib.escapeShellArg cfg.datasourceId} \
          --arg base_url ${lib.escapeShellArg cfg.baseUrl} \
          --rawfile api_key "$CREDENTIALS_DIRECTORY/api_key" \
          --rawfile application_key "$CREDENTIALS_DIRECTORY/application_key" \
          '$existing[0] * {
            port: $port,
            datasource_id: $datasource_id,
            additional_config: {
              base_url: $base_url,
              api_key: ($api_key | rtrimstr("\n")),
              application_key: ($application_key | rtrimstr("\n"))
            }
          }' > ${configFile}.tmp
        mv ${configFile}.tmp ${configFile}
      '';

      serviceConfig = {
        ExecStart = "${lib.getExe cfg.package} ${configFile}";
        Restart = "on-failure";
        RestartSec = 5;

        DynamicUser = true;
        StateDirectory = "weather-station-wrapper";
        UMask = "0077";
        LoadCredential = [
          "api_key:${cfg.apiKeyFile}"
          "application_key:${cfg.applicationKeyFile}"
        ];

        CapabilityBoundingSet = "";
        NoNewPrivileges = true;
        PrivateDevices = true;
        ProtectHome = true;
        ProtectSystem = "strict";
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = [
          "AF_INET"
          "AF_INET6"
          "AF_UNIX"
        ];
        RestrictNamespaces = true;
        LockPersonality = true;
        MemoryDenyWriteExecute = true;
        SystemCallArchitectures = "native";
      };
    };
  };
}
