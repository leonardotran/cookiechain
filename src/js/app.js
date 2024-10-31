
// Import required libraries
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/api');
const fs = require('fs');

// Define the application object
App = {
  account: '',
  cookiechain: {},
  api: null,
  keyring: null,
  privateKey: '',
  publicKey: '',

  // Load the application
  load: async () => {
    await App.loadApi();
    await App.loadAccount();
    await App.loadContract();
  },

  // Load the API for connecting to the Aleph Zero testnet
  loadApi: async () => {
    // Connect to Aleph Zero Testnet (replace with correct endpoint)
    const provider = new WsProvider('wss://testnet.alephzero.org'); // Aleph Zero testnet WebSocket URL
    App.api = await ApiPromise.create({ provider });
    console.log("Connected to Aleph Zero Testnet");
  },

  // Load the account using Keyring
  loadAccount: async () => {
    App.keyring = new Keyring({ type: 'sr25519' });
    // Replace with your account seed or mnemonic
    const seed = 'resist oak face crash bean disorder turn consider also town six elite'; // Replace with your actual seed or mnemonic
    App.account = App.keyring.addFromUri(seed);
    console.log("Account loaded:", App.account.address);
  },

  // Load the smart contract
  loadContract: async () => {
    // Load the ABI from the JSON file
    const abi = JSON.parse(fs.readFileSync('./build/cookie_chain.json', 'utf-8')); // Adjust path as needed
    const contractAddress = '5GTrHGtgq3b4uX9vHmcuCssikFBzdbFXyQkJgKJm6e9EZQNT'; // Replace with your deployed contract address

    // Create a contract instance
    App.cookiechain = new App.api.contracts.Contract(abi, contractAddress);
    console.log("Contract loaded:", contractAddress);

    // Optionally, query the contract state or call functions
    try {
      const { output } = await App.cookiechain.query.cookiesCount(App.account.address, { value: 0 });
      console.log("Cookies Count:", output.toString());
    } catch (error) {
      console.error('Error fetching cookies count:', error);
    }
  },
};

// Start loading the application
App.load().catch(console.error);

// App = {
//   account: '0x0',
//   cookiechain: {},
//   web3: {},
//   privateKey: '',
//   publicKey: '',

//   load: async () => {
//     await App.loadWeb3()
//     await App.loadAccount()
//     await App.loadContract()
//   },

//   // https://medium.com/metamask/https-medium-com-metamask-breaking-change-injecting-web3-7722797916a8
//   loadWeb3: async () => {

//     if (typeof web3 !== 'undefined') {
//         web3 = new Web3(web3.currentProvider);
//     } else {
//         // set the provider you want from Web3.providers
//         web3 = new Web3(new Web3.providers.HttpProvider("http://127.0.0.1:7545"));
//     }
//   },

//   loadAccount: async () => {
//     App.web3 = web3;
//     App.account = await web3.eth.getAccounts();
//   },

//   loadContract: async () => {
//     const networkId = await web3.eth.net.getId()
//     var networkData;
//     var abi;

//     await $.getJSON("../../build/contracts/Cookiechain.json", function (data) {
//       networkData =  data.networks[networkId];
//       abi = data.abi;
//     })
//     if (networkData){
//       const cookiechain = new web3.eth.Contract(abi, networkData.address)
//       App.cookiechain = cookiechain
//     }else{
//       window.alert('Cookiechain contract not deployed to detected network.')
//     }    
//   },

// }


