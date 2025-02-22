<span style="font-family:Amiri;">

# Requirements
I want this app to do a few things for me. First, I want it to be a place that holds all the projects I'm working on i.e. what tasks I have and when I want them to be done. I just don't want to keep going back and forth between differnt things and having to keep on figuring out where I am in a project. Of course, that comes with actutally being consistent and sticking with each of the projects and not procrastinating, but that can also be a bit much sometimes. I also want this project to make sure that you're not lagging behind on certain things i.e. if you haven't touched a task for a project in a few days it should put that at the top of your priority list. Just having something to tell you what you need to do and by when would be good. Ok, I think that freewrite gave me a bit of material to pick some reqiurements from and I think we'll start with:
- Projects should be an encompassing task - an object where tasks for a specific subject are held
    - Folder-file structure architecture for how apps and tasks are related to each other
    - Folder should be able to be made inside of other folders i.e. both files and folders have parents which are both folders
- Prioritzed & weighted tasks
    - tasks should be sent to you in an order of prioritzation and weight. What I mean by weight is that tasks should have their prioritization measured by their relevance. As an example, if you have a high priority task in a project that is mostly complete (low project priority but high task priority) then the final priority should be a weighted aggregate of the two. A low priority task in a project that has had no activity should have its resulting priority be higher than its intial. However, a task's individual priority should outgrow the priority of the project if nothing is happening in it. Either the task should grow linearly or exponentially and maybe those rates can be set by the task's individual priority level.
- History and trends
    - You should be able to see what you have completed for which tasks and how that has progressed over time. 
- Usability
    - Website - this should be pretty universal, make APIs and have it work with a nice UI
    - CLI - I want to have a CLI version just because I feel like that would be fun to have. 
    - ?Mobile?

Ok, with that so far, I'm going to make some folders to start this app and go from there.

# Side Notes
Apparently, **for rust-analyzer to work, you need to open a project with cargo.toml at the root of it**. As nice as the directories in the todo_app project are, if I actually want the IDE experience, I have to open each one of these modules seperately. Keep this in mind future me!

</span>