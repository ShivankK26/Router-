// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;
pragma abicoder v2;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@routerprotocol/evm-gateway-contracts/contracts/IGateway.sol";

contract RouterPay is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    event RouterPayRequest(
        uint256 requestId,
        string dstChainId,
        string recipient,
        uint256 maxAmount
    );
    event RouterPayReceive(string srcChainId, uint256 amount, string recipient);

    address public gatewayAddress;
    string public routerChainId; // router chainid
    IGateway gateway; // gateway instance
    string public routerPayOnRouterChainAddress;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        string memory _routerPayOnRouterChainAddress,
        string memory _routerChainId,
        address payable _gatewayAddress
    ) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        routerChainId = _routerChainId;
        routerPayOnRouterChainAddress = _routerPayOnRouterChainAddress;

        gatewayAddress = _gatewayAddress;
        gateway = IGateway(gatewayAddress);
    }

    function setDappMetadata(
        string memory feePayerAddress
    ) external payable onlyOwner returns (uint256) {
        return gateway.setDappMetadata{value: msg.value}(feePayerAddress);
    }

    /**
     * it enroll other rider nft contract which it deployed on other supported chain, can only be executed by owner of this contract
     * @param _routerPayOnRouterChainAddress  contract address of router pay on router chain
     */
    function updateRouterPayContract(
        string memory _routerPayOnRouterChainAddress
    ) external onlyOwner {
        routerPayOnRouterChainAddress = _routerPayOnRouterChainAddress;
    }

    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyOwner {}

    /**
     * modifer to check if caller is trusted gateway or not
     */
    modifier isGateway() {
        require(msg.sender == gatewayAddress, "Caller: not gateway");
        _;
    }

    function WithdrawFund(
        string memory dstChainId, // dst chain id where user wants route token
        string memory recipient, // who will receive route
        uint256 maxAmount, // route collect till this point of time, if passed 0 then it will withdraw all token till current point
        uint64 streamId,
        bytes calldata requestMetadata
    ) public payable {
        // TODO:: how to take fee from user []
        require(
            keccak256(abi.encodePacked(routerPayOnRouterChainAddress)) !=
                keccak256(abi.encodePacked("")),
            "Invalid dst chain"
        );

        // ACK NOT REQUIRED
        require(
            uint8(requestMetadata[48]) == 0 &&
                keccak256(requestMetadata[50:]) ==
                keccak256(abi.encodePacked("")),
            "invalid asm or ack"
        );
        bytes memory payload = abi.encode(
            dstChainId,
            toBytes(msg.sender),
            recipient,
            streamId,
            maxAmount
        );
        uint256 eventId = gateway.iSend(
            1,
            0, //routeAmount
            "0x", //routeRecipient
            routerChainId,
            requestMetadata,
            abi.encode(routerPayOnRouterChainAddress, payload)
        );

        emit RouterPayRequest(eventId, dstChainId, recipient, maxAmount);
    }

    /**
     * When a cross-chain transfer is made from a router chain to this chain, the gateway contract on the destination chain executes this function.
     * @param requestSender contract address of router chain contract
     * @param srcChainId chain id of src chain
     */
    function iReceive(
        string memory requestSender,
        bytes memory packet,
        string memory srcChainId
    ) external isGateway returns (string memory) {
        require(
            keccak256(abi.encodePacked(requestSender)) ==
                keccak256(abi.encodePacked(routerPayOnRouterChainAddress)),
            "Not called from trusted contract"
        );

        (uint256 amount, string memory recipient) = abi.decode(
            packet,
            (uint256, string)
        );

        emit RouterPayReceive(srcChainId, amount, recipient);
        return "";
    }

    function iAck(
        uint256 eventIdentifier,
        bool execFlag,
        bytes memory _payload
    ) external isGateway {}

    function toBytes(address a) internal pure returns (bytes memory b) {
        assembly {
            let m := mload(0x40)
            a := and(a, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
            mstore(
                add(m, 20),
                xor(0x140000000000000000000000000000000000000000, a)
            )
            mstore(0x40, add(m, 52))
            b := m
        }
    }

    receive() external payable {}
}
