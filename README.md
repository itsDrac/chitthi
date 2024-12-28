#Chitthi

Chithi is an TUI mail manager application build in rust using ratatui crate.
The intended use of this application are:
- Check emails.
- move/copy emails into different folders.
- deleting emails. 
- create a new folder in mailbox.

Currently this application is only supporting "Gmail", but I plan to use different mail provider in future.
To perform all the above task I am using gmail's Imap server. (Sending emails using SMTP server is not currently on the list but I would like to add that in maybe version 2 of this application).

## Working.
To run the application type `cargo run` a welcome screen will come. Press `a` to add new account. Email box would be auto selected, enter email in it. Use `tab` to switch between box. Once password box is selected please enter your **app password for your gmail account** then press tab and enter to add the account.

After adding the acccount you can quit the application using `Esc` key. Please re run the program this time a black sreen will be displayed with avaliable folder names in the top that's `folder_list` section.

when the `folder_list` section is white it means is not active press tab twice to make it active. the idea behind it was I wanted to divide sreen in 3 sections `folder_list`, `subject_view` and `utils_list`so by pressing tab it just cycles around these 3 secctions.

when `folder_list` is selected use `l` key to hover over the given folders, Indecated when text is green. press `Enter` to select a folder. You'll have to do the above process again to select the same folder again (known bug, Dont know how to solve). Once you double select same folder. you'll see latest 5 emails (only the subject of them).

Please perform the codereview.
I am looking forward for the feedback. :)
Thanks.
