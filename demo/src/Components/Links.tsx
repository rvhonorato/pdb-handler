export const Links = () => {
  return (
    <div className="flex flex-wrap gap-4 pt-3 justify-center">
      <a
        href="https://github.com/rvhonorato/pdb-handler"
        target="_blank"
        rel="noopener noreferrer"
        className="px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 rounded-md transition-colors flex items-center"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-5 w-5 mr-2"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path
            fillRule="evenodd"
            clipRule="evenodd"
            d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
          />
        </svg>
        GitHub
      </a>

      <a
        href="https://crates.io/crates/pdb-handler"
        target="_blank"
        rel="noopener noreferrer"
        className="px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 rounded-md transition-colors flex items-center"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-5 w-5 mr-2"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M2.5 7.5v9l9 4.5 9-4.5v-9l-9-4.5-9 4.5zM12 4.75l7.5 3.75v.75L12 13 4.5 9.25v-.75L12 4.75zM12 14.25l7.5-3.75v5.25L12 20l-7.5-3.75v-5.25L12 14.25z" />
        </svg>
        Crates.io
      </a>

      <div
        className="px-4 py-2 bg-gray-50 text-gray-400 rounded-md flex items-center cursor-not-allowed"
        title="Coming soon"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-5 w-5 mr-2 opacity-75"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M1.763 0C.786 0 0 .786 0 1.763v20.474C0 23.214.786 24 1.763 24h20.474c.977 0 1.763-.786 1.763-1.763V1.763C24 .786 23.214 0 22.237 0zM5.13 5.323l13.837.019-.009 13.836h-3.464l.01-10.382h-3.456L12.04 19.17H5.113z" />
        </svg>
        NPM Package <span className="ml-2 text-sm italic">(Coming Soon)</span>
      </div>
      <a
        href="https://docs.rs/pdb-handler"
        target="_blank"
        rel="noopener noreferrer"
        className="px-4 py-2 bg-indigo-50 hover:bg-indigo-100 text-indigo-700 rounded-md transition-colors flex items-center"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className="h-5 w-5 mr-2"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z" />
          <path d="M11 11h2v6h-2zm0-4h2v2h-2z" />
        </svg>
        API Documentation
      </a>
    </div>
  );
};
