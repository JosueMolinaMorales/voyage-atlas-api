# voyage-atlas-api
The API for the Voyage Atlas Social Media. A social media for travel enthusiasts

VoyageAtlas API is a RESTful web service that powers the VoyageAtlas platform, a social media platform for travel enthusiasts. It allows users to share their travel experiences, discover new destinations, and connect with like-minded adventurers.

## Features

- [ ] **User Authentication**: Users can sign up and log in to create their profiles and start sharing their travel stories.
- [ ] **Travel Posts and Stories**: Users can create engaging travel posts and stories, complete with text, images, and location tags.
- [ ] **Hashtags and Tagging**: Categorize travel posts with hashtags and tag other users in their stories.
- [ ] **Explore and Discover**: Users can explore and discover travel posts from other adventurers based on destinations and interests.
- [ ] **Interactions and Engagement**: Engage with the community by liking, commenting, and sharing travel posts.
- [ ] **Travel Planning Tools**: Integrated tools for weather forecasts, travel itineraries, and currency conversion to assist in trip planning.
- [ ] **Rating and Reviews**: Users can rate and review destinations, accommodations, and activities they've experienced.
- [ ] **Privacy and Moderation**: Users can control the visibility of their posts, and content is moderated to maintain a positive community.
- [ ] **Map Integration**: Visualize travel routes and marked places visited by users on interactive maps.
- [ ] **Events and Meetups**: Users can organize and join travel-related events and meetups.
- [ ] **Personalized Recommendations**: Recommendation engine suggests travel destinations, activities, and fellow travelers based on user preferences.
- [ ] **Data Backup and Recovery**: Regular data backups ensure users' travel stories and data are secure.

## Getting Started

Follow these steps to set up the VoyageAtlas API on your local machine:

1. **Prerequisites**: Make sure you have Rust and PostgreSQL installed on your system.

2. **Clone the Repository**: Clone this repository to your local machine.

3. **Set Up PostgreSQL**: Create a PostgreSQL database and update the database configuration in the `.env` file.

4. **Install Dependencies**: Run `cargo build` to install the project dependencies.

5. **Run Migrations**: Run `diesel migration run` to set up the initial database schema.

6. **Start the Server**: Run `cargo run` to start the API server.

7. **Explore the API**: Access the API at `http://localhost:8000` and use tools like `curl` or Postman to interact with the endpoints.

## Contributing

We welcome contributions to make VoyageAtlas even better! If you have any bug fixes, new features, or improvements, feel free to submit a pull request.

## License

VoyageAtlas API is open-source and released under the [MIT License](LICENSE).

Happy traveling and sharing your adventures with VoyageAtlas!
