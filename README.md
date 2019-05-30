# onboarding-rust

### First review if you have set up your Joy Labs [development setup](https://honey.is/home/#post/778734)

1.- [Create an SSH key on your local computer.](https://help.github.com/en/enterprise/2.15/user/articles/adding-a-new-ssh-key-to-your-github-account)

2.- [Check if you have git installed, if not install it:](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
```
git --version
```

3.- On the folder you select, clone the master repository with SSH
```
git clone SSH_link
```

4.- Check repository branches and in wich one you are:
```
git branch
```

5.- Create and move to a new branch with your name (this will be your master personal repository):
```
git checkout -b your_name
```

6.- Create and move to a new branch for your exercice:
```
git checkout -b your_name-e1
```

7.- Solve your exercice, follow the structure of the folders and 
*To run the test navigate to that days submission and run*

```
cargo test
```

8.- Add all your changes (you can review your modified files with *git status*):
```
git add .
```

9.-Create your commit:
```
git commit -m "your comments here"

```

10.- Push changes on your branch:
```
git push origin name_of_the_branch
```

11.- On Github make a pull requeset of your branch and choose a peer to review it (try to choose someone different).


### [Cool terminal configuaration (Optional)](https://medium.com/the-code-review/make-your-terminal-more-colourful-and-productive-with-iterm2-and-zsh-11b91607b98c)




