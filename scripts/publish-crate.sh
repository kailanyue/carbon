#!/usr/bin/env bash

set -ex

workspace_crates=(
    carbon-macros
    carbon-proc-macros
    carbon-core
    carbon-test-utils

    carbon-helius-atlas-ws-datasource
    carbon-rpc-block-subscribe-datasource
    carbon-rpc-program-subscribe-datasource
    carbon-rpc-transaction-crawler-datasource
    carbon-yellowstone-grpc-datasource

    carbon-log-metrics
    carbon-prometheus-metrics

    carbon-drift-v2-decoder
    carbon-fluxbeam-decoder
    carbon-jupiter-dca-decoder
    carbon-jupiter-limit-order-2-decoder
    carbon-jupiter-limit-order-decoder
    carbon-jupiter-perpetuals-decoder
    carbon-jupiter-swap-decoder
    carbon-kamino-lending-decoder
    carbon-kamino-vault-decoder
    carbon-lifinity-amm-v2-decoder
    carbon-memo-program-decoder
    carbon-meteora-dlmm-decoder
    carbon-moonshot-decoder
    carbon-mpl-core-decoder
    carbon-mpl-token-metadata-decoder
    carbon-name-service-decoder
    carbon-okx-dex-decoder
    carbon-openbook-v2-decoder
    carbon-orca-whirlpool-decoder
    carbon-phoenix-v1-decoder
    carbon-pumpfun-decoder
    carbon-raydium-amm-v4-decoder
    carbon-raydium-clmm-decoder
    carbon-raydium-cpmm-decoder
    carbon-raydium-liquidity-locking-decoder
    carbon-sharky-decoder
    carbon-spl-associated-token-account-decoder
    carbon-stabble-stable-swap-decoder
    carbon-stabble-weighted-swap-decoder
    carbon-stake-program-decoder
    carbon-system-program-decoder
    carbon-token-2022-decoder
    carbon-token-program-decoder
    carbon-zeta-decoder
)

for crate in "${workspace_crates[@]}"; do
   echo "--- $crate"
   cargo package -p $crate
   cargo publish -p $crate
done