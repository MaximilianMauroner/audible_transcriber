# Liberate Bookmarks

Liberate Bookmarks is a web application designed to export your audio bookmarks in away that they can be importet into Notion/Excel/Sheets.
This is done leveraging the power of SvelteKit and Tauri for a smooth, native experience.

# Installation

To install the application locally, download the latest release from the releases page.

## Development

To get started with Liberate Bookmarks, clone the repository and install the dependencies:

```sh
git clone https://github.com/MaximilianMauroner/liberate_bookmarks
cd liberate_bookmarks
npm install
```

Running the Application
To run the application in development mode, use the following command:

```sh
npm run tauri dev
```

For building the application for production, use:

```sh
npm run tauri build
```

Dependencies
SvelteKit for the frontend framework.
Tauri for creating a lightweight, native application.
Vite for building and serving the web application.
For a full list of dependencies, refer to the package.json file.

# TODO

- [X] Remove/Ignore bookmarks
- [ ] Fix uninstall problem where 1 folder remains
- [ ] Add settings/config option
- [ ] Make faster
