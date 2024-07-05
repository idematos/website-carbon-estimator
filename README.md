# Website Carbon Estimator

This is a tool to estimate the carbon emissions of a website using the [CO2.js](https://developers.thegreenwebfoundation.org/co2js/overview/) library. It calculates the carbon footprint of a website visit by analyzing the data transfer and energy consumption.

## Usage

To estimate the carbon emissions of a website, you need to run the program with the website URL. The default URL is set to [https://www.mogrene.com](https://www.mogrene.com). You can change it in the `src/bin/main.rs` file by modifying the following line:

```
let website_url = "https://www.mogrene.com"; // Change this to the URL you want to check
```

Then, run the program:

```
cargo run
```


## Output

The program will output the carbon emissions data, including the amount of data transferred, energy consumption, and carbon emissions.


## License

Licensed under the [MIT License](https://opensource.org/license/MIT).
