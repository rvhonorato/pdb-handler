export const Footer = () => {
  return (
    <footer className="mt-16 bg-gray-50 border-t border-gray-100">
      <div className="max-w-5xl mx-auto px-4 py-8">
        <div className="md:flex md:items-center md:justify-between">
          <div className="space-y-4 mb-8 md:mb-0 md:max-w-sm">
            <h3 className="text-lg font-semibold text-gray-900">pdb-handler</h3>
            <p className="text-sm text-gray-600">
              A modern, fast, compatible and open source approach to structural
              biology data processing.
            </p>
          </div>

          <nav className="grid grid-cols-2 gap-8 sm:grid-cols-3">
            <div className="space-y-2">
              <h4 className="text-sm font-semibold text-gray-900">Resources</h4>
              <ul className="space-y-1">
                <li>
                  <a
                    href="https://github.com/rvhonorato/pdb-handler"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    GitHub Repository
                  </a>
                </li>
                <li>
                  <a
                    href="https://crates.io/crates/pdb-handler"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Crates.io Page
                  </a>
                </li>
                <li>
                  <div
                    className="text-sm text-gray-400
                    "
                  >
                    NPM Package (soon!)
                  </div>
                </li>
                <li>
                  <a
                    href="https://docs.rs/pdb-handler"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    API Documentation
                  </a>
                </li>
              </ul>
            </div>

            <div className="space-y-2">
              <h4 className="text-sm font-semibold text-gray-900">
                Contribute
              </h4>
              <ul className="space-y-1">
                <li>
                  <a
                    href="https://github.com/rvhonorato/pdb-handler/issues"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Report Issues
                  </a>
                </li>
                <li>
                  <a
                    href="https://github.com/rvhonorato/pdb-handler/pulls"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    Submit PRs
                  </a>
                </li>
              </ul>
            </div>

            <div className="space-y-2">
              <h4 className="text-sm font-semibold text-gray-900">Connect</h4>
              <ul className="space-y-1">
                <li>
                  <a
                    href="https://github.com/rvhonorato"
                    className="text-sm text-gray-600 hover:text-indigo-600 transition-colors"
                    target="_blank"
                  >
                    Contact
                  </a>
                </li>
              </ul>
            </div>
          </nav>
        </div>

        <div className="mt-8 pt-8 border-t border-gray-200">
          <p className="text-xs text-gray-600 text-center">
            &copy; {new Date().getFullYear()} pdb-handler - Created by Rodrigo
            V. Honorato
            <span className="block mt-1">MIT Licensed</span>
          </p>
        </div>
      </div>
    </footer>
  );
};
