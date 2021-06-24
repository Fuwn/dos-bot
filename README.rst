.. raw:: html

  <h1 align="center"><code>dos-bot</code></h1>

  <p align="center">

.. image:: https://img.shields.io/discord/854071194453671976
  :alt: Discord
  :target: https://discord.io/assembly

.. image:: https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg
  :alt: Say Thanks
  :target: https://saythanks.io/to/fuwnzy@gmail.com

.. image:: https://img.shields.io/github/license/fuwn/dos-bot
  :alt: License
  :target: ./LICENSE

.. raw:: html

  </p>

Usage
-----

Docker
~~~~~~

.. code-block:: shell

  $ docker run --name DosBot -v /var/lib/dos-bot:/.dos-bot fuwn/dos-bot

Docker Compose
~~~~~~~~~~~~~~

.. code-block:: shell

  $ docker-compose up -d

Invite
~~~~~~

https://discord.com/api/oauth2/authorize?client_id=857695800677892106&permissions=8&scope=bot

Development Dependencies
------------------------

Required
~~~~~~~~

- `Rust <https://www.rust-lang.org/>`_ — The backbone of it all.
- `cargo-make <https://github.com/sagiegurari/cargo-make>`_ — Cross-platform
  `make <https://www.gnu.org/software/make/>`_ substitute

Optional
~~~~~~~~

- `cargo-watch <https://crates.io/crates/cargo-watch>`_ — Recompilation on file-system changes

*These development dependencies (excluding sqlfluff) will automatically be satisfied if you are
using the Nix shell configuration as provided.*

License
~~~~~~~

`GNU General Public License v3.0 <./LICENSE>`_
