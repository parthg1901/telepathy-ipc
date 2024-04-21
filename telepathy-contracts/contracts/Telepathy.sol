// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

import "@zondax/filecoin-solidity/contracts/v0.8/types/CommonTypes.sol";
import "@zondax/filecoin-solidity/contracts/v0.8/utils/Misc.sol";
import "@zondax/filecoin-solidity/contracts/v0.8/utils/Actor.sol";
import "./CBOR.sol";

contract Telepathy {
    using CBOR for CBOR.CBORBuffer;

    bytes public raw_response;
    bytes public int_response;
    bytes public game_data;
    address[2] private players;
    int256 public num_response;
    bytes public buffer;

    constructor () public {
        game_data = Actor.callByID(
            CommonTypes.FilActorId.wrap(49), //uint64, actor id
            1251119395, // method number
            Misc.NONE_CODEC,
            new bytes(0),
            0,
            false
        );
        players[0] = (msg.sender);
    }

    function join() public {
        if (msg.sender != players[0]) {
            players[1] = msg.sender;
        } else {
            revert();
        }
    }

    function rejoin() public view returns (uint256){
        if (msg.sender == players[0]) {
            return 1;
        } else if (msg.sender == players[1]) {
            return 2;
        }
        return 0;
    }

    function interact(uint64 player_num, uint64 sensei) public {

        uint256 capacity = 1;

        CBOR.CBORBuffer memory buf = CBOR.create(capacity);
        CBOR.startFixedArray(buf, 2);
        CBOR.writeUInt64(buf, (player_num));
        CBOR.writeUInt64(buf, (sensei));

        int_response = Actor.callByID(
            CommonTypes.FilActorId.wrap(49), //uint64, actor id
            3572881031, // method number
            Misc.CBOR_CODEC,
            buf.data(),
            0,
            false
        );
    }

    function get_lives() public returns (bytes memory) {
        raw_response = Actor.callByID(
            CommonTypes.FilActorId.wrap(49), //uint64, actor id
            4166346685, // method number
            Misc.NONE_CODEC,
            new bytes(0),
            0,
            true
        );


        return raw_response;
    }
}