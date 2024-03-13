# Currency Converter

This is a currency converter written in Rust. Use it with your favorite terminal. 

## Prerequisites

You need get an API key from [exchangerate-api.com](https://www.exchangerate-api.com/) and set it in the `.env` file.
You will need to register and get an FREE API key from the exchangerate-api.com website. Once you have the API key, create a `.env` file in the root directory of the project and add the following line to it:

```sh
API_KEY=your_api_key
```

## Building the Project

To build the project, run the following command in your terminal:

```sh
cargo build
```

## Running the Project
To run the project, run the following command in your terminal:

```sh   
cargo run
```

## How to Use

With down / up arrow keys, select the option You wanna use. 
You can list all currencies symbols, convert between currencies or exit the program.
When you select the convert option, The program will prompt you to enter the base currency, the target currency and the amount. After entering the required information, the program will display the converted amount.
