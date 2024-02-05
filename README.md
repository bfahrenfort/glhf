# GLHF - Good Luck, Here's the FAQ
For when you just can't bother to google for the docs. 

Simply type `glhf <myfavoriteprogram>` to be taken right to its documentation in your default web browser or pager.
 
## Features
- Local/offline database for faster fetches
- Community supported - if glhf hasn't tracked your favorite program yet, you can request an addition right from your terminal
    - Request updates too!
- Many different types of documentation: GitHub Wiki links (`Github` type), `man` pages (`Man` type), and any conventional docs website (`Book` type)

## Installing
`cargo install glhf`

## Usage
`glhf <program_name>`: opens documentation for the argument. If there is none, you will be prompted to optionally create a queue entry for approval.

`glhf create <name> [doctype] [url]`: create a queue entry for the specified program. If a doctype (and if required, a URL) is not supplied, you will be prompted to provide them.

## Supported Programs
- TODO: a [Program Database]() fetching from the db to display all entries in a web ui
