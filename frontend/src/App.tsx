import { Component, createSignal, JSX } from "solid-js";

const App: Component = () => {
    return (
        <div>
            <UploadForm />
        </div>
    );
};

const UploadForm = () => {
    const [aaxFile, setAaxFile] = createSignal<File | null>(null);
    const [jsonFile, setJsonFile] = createSignal<File | null>(null);

    console.log(aaxFile(), jsonFile());

    const handleAaxUpload: JSX.EventHandler<HTMLInputElement, InputEvent> = (
        event
    ) => {
        if (event.target.files.length > 0) setAaxFile(event.target.files[0]);
    };
    const handleJsonUpload: JSX.EventHandler<HTMLInputElement, InputEvent> = (
        event
    ) => {
        if (event.target.files.length > 0) setJsonFile(event.target.files[0]);
    };

    const removeAAxFile = () => {
        setAaxFile(null);
    };
    const removeJsonFile = () => {
        setJsonFile(null);
    };

    const handleSubmit = (e: Event) => {
        e.preventDefault();
    };
    return (
        <div class="min-h-screen h-full dark:bg-gray-900 flex justify-center items-center">
            <div class="w-full max-w-md bg-white shadow-lg rounded-lg px-4 py-6 space-y-4">
                <div class="space-y-1">
                    <h1 class="text-xl font-bold text-gray-900">
                        Upload AAX and JSON Files
                    </h1>
                    <p class="text-gray-500">
                        Please upload an AAX file and a JSON file to continue.
                    </p>
                </div>
                <form class="space-y-4" onSubmit={handleSubmit}>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">
                            AAX File
                        </label>
                        <div class="mt-1">
                            <div class="flex justify-between items-center border border-gray-300 bg-white rounded-md px-3 py-2">
                                <span class="text-sm text-gray-500 truncate">
                                    {aaxFile()
                                        ? aaxFile()?.name
                                        : "No file selected"}
                                </span>
                                <input
                                    type="file"
                                    name="aaxFile"
                                    accept=".aax"
                                    class="sr-only"
                                    onChange={handleAaxUpload}
                                />
                                <button
                                    type="button"
                                    class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                    onClick={() =>
                                        document
                                            .getElementsByName("aaxFile")[0]
                                            .click()
                                    }
                                >
                                    Choose
                                </button>
                            </div>
                        </div>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">
                            JSON File
                        </label>
                        <div class="mt-1">
                            <div class="flex justify-between items-center border border-gray-300 bg-white rounded-md px-3 py-2">
                                <span class="text-sm text-gray-500 truncate">
                                    {jsonFile()
                                        ? jsonFile()?.name
                                        : "No file selected"}
                                </span>
                                <input
                                    type="file"
                                    name="jsonFile"
                                    accept=".json"
                                    class="sr-only"
                                    onChange={handleJsonUpload}
                                />
                                <button
                                    type="button"
                                    class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                    onClick={() =>
                                        document
                                            .getElementsByName("jsonFile")[0]
                                            .click()
                                    }
                                >
                                    Choose
                                </button>
                            </div>
                        </div>
                    </div>
                    <div>
                        <button
                            type="submit"
                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                            disabled={!aaxFile() || !jsonFile()}
                        >
                            Upload
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};

export default App;
