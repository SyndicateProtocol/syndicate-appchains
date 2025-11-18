import { deployTeeModule } from "../features/deployTeeModule";

async function main() {
	const addresses = await deployTeeModule();
	console.log(addresses);
}

main().catch((error) => {
	console.error(error);
	process.exit(1);
});
