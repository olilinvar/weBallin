#include <SFML/Graphics.hpp>
#include <iostream>

int main() {
    // Create a window
    sf::RenderWindow window(sf::VideoMode(800, 600), "SFML Window");

    // Load the texture from a 32x32 PNG file
    sf::Texture texture;
    if (!texture.loadFromFile("../../")) {
        // Handle loading error
        return 1;
    }

    // Create a sprite with the loaded texture
    sf::Sprite sprite(texture);
    // Set the position of the sprite
    sprite.setPosition(400.f, 300.f); // Center of the window

    // Main loop
    while (window.isOpen()) {
        // Process events
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                // Close the window if the close button is clicked
                window.close();
            }
        }

        // Clear the window
        window.clear();

        // Draw the sprite
        window.draw(sprite);

        // Display the contents of the window
        window.display();
    }

    return 0;
}
