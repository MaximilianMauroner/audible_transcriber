/* @refresh reload */
import { render } from "solid-js/web";
import {
    QueryClient,
    QueryClientProvider,
    createQuery,
} from "@tanstack/solid-query";
import { Switch, Match, For } from "solid-js";

const queryClient = new QueryClient();

import "./index.css";
import App from "./App";

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        "Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?"
    );
}

render(
    () => (
        <QueryClientProvider client={queryClient}>
            <App />
        </QueryClientProvider>
    ),
    root!
);
