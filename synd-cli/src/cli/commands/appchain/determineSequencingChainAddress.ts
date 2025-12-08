import { print } from "@/utils/print";
import type { Command } from "@commander-js/extra-typings";
import { getSequencingChainAddress } from "@/utils/forwarderHelper";
import { risa, syndicate } from "@/utils/constants";

export function determineSequencingChainAddressCommand(program: Command) {
  program
    .command("get-addr")
    .description("Determine the sequencing chain address for a given chain ID")
    .argument(
      "<chainId>",
      "The chain ID to determine the sequencing chain address for"
    )
    .option("-t, --testnet", "Indicates that the chain is a testnet")
    .action(async (chainIdStr: string, options: { testnet?: boolean }) => {
      const chainId = Number(chainIdStr);
      if (Number.isNaN(chainId)) {
        return print("Invalid chain ID");
      }
      const sequencingChainAddress = getSequencingChainAddress(
        chainId,
        options.testnet ? risa.id : syndicate.id
      );
      print("Sequencing chain address:", sequencingChainAddress);
    });
}
