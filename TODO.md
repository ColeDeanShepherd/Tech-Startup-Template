# To-Do

## High-Priority
* User sign-ups
  * Server
    * Create migration for user table
        * How to store passwords??
    * Flesh out `SignUpAction`
    * Create executor for `SignUpAction`
        * Ensure user doesn't already exist
        * Hash password
        * Handle errors correctly
        * Log account creation somehow (log request? internal log?)
        * Use Diesel to INSERT into users table
    * Call executor in sign up endpoint
    * TESTS!
  * Client
    * Create UI for user sign ups
    * TESTS?

## Low-Priority
* Move REST API layer into its own project, separate from underlying logic.
* Get PostgreSQL running in a docker container
* Improve diesel getting started docs for Windows (add to path)