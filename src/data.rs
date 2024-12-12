use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55\" 4K UHD Smart TV".to_string(),
            price: 499.99,
            description: "Experience stunning picture quality with the Samsung 55\" 4K UHD Smart TV. Equipped with smart features and a sleek design, it's perfect for movie nights or gaming.".to_string(),
            image: "/samsung_tv.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple AirPods Pro (2nd Gen)".to_string(),
            price: 249.99,
            description: "Enjoy crystal-clear sound and active noise cancellation with the Apple AirPods Pro. The second-generation model offers improved audio quality and a more secure fit.".to_string(),
            image: "/airpods_pro.png".to_string()
        },
        Product {
            id: 3,
            name: "Sony PlayStation 5 Console".to_string(),
            price: 499.99,
            description: "Dive into the next generation of gaming with the Sony PlayStation 5 Console. Featuring ultra-fast load times and breathtaking graphics, it's the ultimate gaming experience.".to_string(),
            image: "/ps5_console.png".to_string()
        },
        Product {
            id: 4,
            name: "Microsoft Surface Laptop 5".to_string(),
            price: 899.99,
            description: "Power through your tasks with the Microsoft Surface Laptop 5. With a 12th-gen Intel processor and a stunning PixelSense display, this laptop is built for productivity.".to_string(),
            image: "/surface_laptop.png".to_string()
        },
        Product {
            id: 5,
            name: "Samsung Galaxy S23 Ultra".to_string(),
            price: 1199.99,
            description: "Capture professional-quality photos and enjoy an ultra-responsive screen with the Samsung Galaxy S23 Ultra. Powered by the latest Snapdragon processor, it's the ultimate smartphone.".to_string(),
            image: "/galaxy_s23.png".to_string()
        },
        Product {
            id: 6,
            name: "Bose QuietComfort 45 Headphones".to_string(),
            price: 329.99,
            description: "Immerse yourself in exceptional sound and silence with the Bose QuietComfort 45 Headphones. Featuring noise-cancelling technology and superior comfort for long listening sessions.".to_string(),
            image: "/bose_headphones.png".to_string()
        },
        Product {
            id: 7,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 999.99,
            description: "The Dell XPS 13 combines power, portability, and a stunning display in one sleek package. Perfect for both work and play, it’s a laptop built to meet all your needs.".to_string(),
            image: "/dell_xps.png".to_string()
        },
        Product {
            id: 8,
            name: "Logitech MX Master 3S Mouse".to_string(),
            price: 99.99,
            description: "The Logitech MX Master 3S is the perfect mouse for productivity and precision. With an ergonomic design and customizable buttons, it's ideal for both work and creative tasks.".to_string(),
            image: "/logitech_mouse.png".to_string()
        },
        Product {
            id: 9,
            name: "Nikon Z6 II Mirrorless Camera".to_string(),
            price: 1999.99,
            description: "Capture stunning photos and videos with the Nikon Z6 II Mirrorless Camera. Featuring a 24.5 MP full-frame sensor and advanced autofocus, it’s perfect for both professionals and enthusiasts.".to_string(),
            image: "/nikon_z6.png".to_string()
        },
        Product {
            id: 10,
            name: "Ring Video Doorbell 4".to_string(),
            price: 199.99,
            description: "Keep your home secure with the Ring Video Doorbell 4. Featuring HD video, two-way talk, and motion detection, it allows you to monitor your front door from anywhere.".to_string(),
            image: "/ring_doorbell.png".to_string()
        }
    ]
}
