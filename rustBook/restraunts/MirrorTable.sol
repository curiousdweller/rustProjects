pragma solidity 0.8.7;
// What is the value? Interface for investors to easily track their investments into start ups and manage thier portfolio.
// Eventually become the vehicle for making investments.
contract MirrorTableFactory {
    // Import Open Zeppelin Counter Module


    struct CapTable {
        // Some variable to capture the name of the company
        address founder;
        uint256 totalSupply;  //Pack into less storage
        uint256 valuation;
        uint256 id;
    }

    modifier onlyFounder(uint256 _id) {
        require(idToCapTable[_id].founder == msg.sender, "Only Founder");
        _;
    }

    modifier onlyWhitelisted(uint256 _id) {
        require(whiteList[_id][msg.sender], "Not whitelisted");
        _;
    }


    // Uint256 => Id number of given cap table. Then Second uint256 checks investor address
    mapping(uint256 => mapping(address => uint256)) balance;
    mapping(uint256 => mapping(address => bool)) whiteList;
    mapping(address => uint256) portfolioValue;
    mapping(uint256 => CapTable) idToCapTable;
    function newCapTable() public returns(uint256 id) {
        uint id = 5;


    }

    function addShares(uint256 _id, uint256 _shares) public onlyFounder(_id) {
        // read cap table struct from storage
        // Update total supply => Shareholders become more diluted, same number of shares worth less
    }

    function withdraw(uint256 _id) public payable {
        // Transfer all ether to investor
    }

    function addToWhiteList(uint256 _id, address[] calldata investors) public onlyFounder(_id) {
        // Loop Through Address[] and add to whitelist.
    }

    // Keep a given investors project ids stored on cache
    function getPortfolioValue(uint256[] calldata projects) public view {
        uint value;
        for(uint i; i < projects.length; i++) {
            // Bring captable total supply on to memory
            // Get Investor share of total shares
            // Valuation * investor Share / Total shares = Portfolio value 
        }
    }


}

// Keep the whitelist. If a bunch of people want to crowdfund they can do a multisig
// Or some other kind of investment vehicle.
// Integrate ENS
// The front end is what really matters.