use anyhow::Result;
use std::path::Path;

/// Generate a local typed codec/client probe; it makes no RPC request or transaction.
pub fn generate_client_probe(directory: &Path) -> Result<()> {
    std::fs::write(
        directory.join("probe.ts"),
        r#"import { createPublicClient, defineChain, encodeFunctionData, getAddress, http, parseAbi, type Address } from 'viem';
const recipient: Address = getAddress('0x0000000000000000000000000000000000000001');
const abi = parseAbi(['function transfer(address recipient, uint256 amount) returns (bool)']);
const encoded = encodeFunctionData({ abi, functionName: 'transfer', args: [recipient, 1n] });
const expected = '0xa9059cbb00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001';
if (encoded !== expected) throw new Error('ERC-20 ABI encoding differs from frozen bytes');
const chain = defineChain({ id: 31337, name: 'EVE_B0_API_ONLY', nativeCurrency: { name: 'Fake EVE', symbol: 'EVE_TEST', decimals: 18 }, rpcUrls: { default: { http: ['http://127.0.0.1:1'] } } });
const client = createPublicClient({ chain, transport: http() });
if (client.chain.id !== 31337) throw new Error('Client did not bind chain identity');
console.log('EVE_B0_CLIENT_API_OK');
"#,
    )?;
    Ok(())
}
