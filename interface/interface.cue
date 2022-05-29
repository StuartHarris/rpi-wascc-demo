package interface

import (
	"dagger.io/dagger"
	"dagger.io/dagger/core"
	"universe.dagger.io/docker"
)

// build with cargo
#Build: {
	// source code
	source: dagger.#FS

	_run: docker.#Build & {
		steps: [
			docker.#Pull & {
				source: "rust:latest"
			},

			docker.#Run & {
				command: {
					name: "rustup"
					args: ["component", "add", "rustfmt"]
				}
			},

			docker.#Copy & {
				dest:     "/src"
				contents: source
			},

			docker.#Run & {
				command: {
					name: "cargo"
					args: ["build"]
				}
				workdir: "/src/rust"
			},
		]
	}
	contents: core.#Copy & {
		input:    dagger.#Scratch
		contents: _run.output.rootfs
		source:   "/src/rust/target/*"
		include: ["liboled_interface.*"]
	}
}