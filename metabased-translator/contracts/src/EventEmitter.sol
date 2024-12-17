// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

contract EventEmitter {


function emitEvent1(bytes32 signatureHash, bytes32 nonIndexed) public {
    assembly {
      mstore(0, nonIndexed)
      log1(0, 0x20, signatureHash)
    }
  }

function emitEvent2(bytes32 signatureHash, bytes32 indexed1, bytes32 nonIndexed) public {
    assembly {
      mstore(0, nonIndexed)
      log2(0, 0x20, signatureHash, indexed1)
    }
  }

  function emitEvent3(bytes32 signatureHash, bytes32 indexed1, bytes32 indexed2, bytes32 nonIndexed) public {
    assembly {
      mstore(0, nonIndexed)
      log3(0, 0x20, signatureHash, indexed1, indexed2)
    }
  }

  function emitEvent4(bytes32 signatureHash, bytes32 indexed1, bytes32 indexed2, bytes32 indexed3, bytes32 nonIndexed) public {
    assembly {
      mstore(0, nonIndexed)
      log4(0, 0x20, signatureHash, indexed1, indexed2, indexed3)
    }
  }
}
