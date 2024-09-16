SRC_DIR := $(shell "pwd")
TARGET_DIR := $(shell "pwd")/target

CARGO := cargo
CARGOFLAGS :=
CARGO_TOML := $(SRC_DIR)/Cargo.toml

# this ensures `all` is run by default despite not being the first target in the Makefile
.DEFAULT_GOAL := all

.PHONY: validate_cargo

validate_cargo:
	$(if \
		$(shell which $(CARGO)),\
		$(info Cargo located at $(shell command -v $(CARGO))),\
		$(error Cargo not found in PATH, but is required for build))

# run with DEBUG=1 to use debug configuration
ifneq "$(DEBUG)" "1"
CARGOFLAGS += --release
endif

.PHONY: rv peer
.PHONY: clean


#
# All targets
#
all: rv peer


#
# Compile the rendezvous server executable
#

rv: $(CARGO_TOML) | validate_cargo
	$(CARGO) build $(CARGOFLAGS) --manifest-path=$(CARGO_TOML) --bin=rv


#
# Compile the peer client executable
#

peer: $(CARGO_TOML) | validate_cargo
	$(CARGO) build $(CARGOFLAGS) --manifest-path=$(CARGO_TOML) --bin=peer

#
# Remove build artifacts
#

clean:
	rm -rf $(TARGET_DIR)
