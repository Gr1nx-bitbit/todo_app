<span style="font-family:amiri;">

Ok, I think we should start with just making some mock data for this db. I want to have a ROOT folder which houses a few other folders, and some tasks. I also want some subfolders just to show that this works. We're basically making a linked list but instead of just having one next node we point to, we'll have an array for subfolders and an array for tasks belonging to the current folder.

Another thing is I have to decide on a language to code this in... I can do Go or Rust or some Python? I've already done enough with Python though and I'll get more experience with it doing AI. I think we should go with Go for now and then try and port to Rust later. 

Ok, let's make the data types that are needed, namely, folders and tasks. Once again, folders have an array for subfolders and tasks while also having a parent folder. Tasks on the other just hold a data type for storage and then a parent folder. I want to have the folders be pointers but i also don't want just anyone to be able to edit them... why can't go have private functions ;(

Ok, so I've made sure the Folder types and Task types work on the Go side of things. I need to now make some database operations to extract info and then use those Go types to represent them during runtime. Setting up another db won't be too hard and I'll just use MariaDB again since that's what I have as an image. I'll have to figure out how to connect to it using Go though... the last times were not amazing lol.  

</span>
