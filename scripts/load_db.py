import psycopg2
from faker import Faker
import bcrypt
import random

# Connect to an existing database
CONN = psycopg2.connect("dbname=voyage_atlas user=postgres password=password host=localhost port=5432")

# Open a cursor to perform database operations
CURSOR = CONN.cursor()
FAKER = Faker()

def main():
    global CONN, CURSOR, FAKER
    try:
        users = create_users()
        create_follows(users)
        posts = create_posts(users)
        create_comments(users, posts)
        create_likes(users, posts)
    except Exception as e:
        print("[ERROR] " + str(e))
        # Delete all data from tables
        clear_tables()
    finally:
        # Close communication with the database
        CURSOR.close()
        CONN.close()


def create_users():
    # Users to insert into table
    print("Creating users...", end="")
    users = []
    for _ in range(0, 50):
        uuid = FAKER.unique.uuid4()
        password = FAKER.password()
        # Salt and hash the password
        hsh_pwd = bcrypt.hashpw(password.encode('utf-8'), bcrypt.gensalt())
        users.append({
            'first_name': FAKER.first_name(),
            'email': FAKER.unique.email(),
            'id': uuid,
            'username': FAKER.unique.user_name(),
            'password': hsh_pwd,
            'description': FAKER.text(),
            'last_name': FAKER.last_name(),
            'created_at': FAKER.date_time(),
        })

    print("Done")
    print("Inserting users into database...", end="")
    # Insert users into table
    for user in users:
        CURSOR.execute("""
            INSERT INTO 
                    users (id, email, username, password, first_name, last_name, description, created_at) 
                    VALUES (%s, %s, %s, %s, %s, %s, %s, %s)
            """,
            (user['id'], user['email'], user['username'], user['password'], user['first_name'], user['last_name'], user['description'], user['created_at'])
        )

    # Commit changes to database
    CONN.commit()
    print("Done")

    # Return the users
    return users

def create_follows(users):
    # Make some users follow other users
    print("Creating follows...", end="")
    follows = []
    while len(follows) < 50:
        # select a random user
        user = random.choice(users)
        # select a random user to follow
        follow = random.choice(users)
        # Make sure the user isn't following themselves
        # and that they aren't already following the user
        if user['id'] == follow['id']:
            continue
        if any(follow['id'] == f['user_id'] for f in follows):
            continue
        follows.append({
            'follower': user['id'],
            'user_id': follow['id'],
        })

    print("Done")
    print("Inserting follows into database...", end="")
    # Insert follows into table
    for follow in follows:
        CURSOR.execute("""
            INSERT INTO 
                    users_followers (follower_id, user_id) 
                    VALUES (%s, %s)
            """,
            (follow['follower'], follow['user_id'])
        )

    # Commit changes to database
    CONN.commit()
    print("Done")

def create_posts(users):   
    # Create some posts and assign them to users as authors
    print("Creating posts...", end="")
    posts = []
    for i in range(0, 50):
        uuid = FAKER.uuid4()
        # select a random user
        user = users[i % len(users)]
        posts.append({
            'id': uuid,
            'title': FAKER.sentence(),
            'location': FAKER.city(),
            'created_at': FAKER.date_time(),
            'content': FAKER.text(),
            'author': user['id'],
        })

    print("Done")
    print("Inserting posts into database...", end="")
    # Insert posts into table
    for post in posts:
        CURSOR.execute("""
            INSERT INTO 
                    posts (id, title, location, created_at, content, author) 
                    VALUES (%s, %s, %s, %s, %s, %s)
            """,
            (post['id'], post['title'], post['location'], post['created_at'], post['content'], post['author'])
        )

    # Commit changes to database
    CONN.commit()
    print("Done")

    # Return the posts
    return posts

def create_comments(users, posts):
    # Add some comments to random posts
    print("Creating comments...", end="")
    comments = []
    while len(comments) < 50:
        uuid = FAKER.unique.uuid4()
        # select a random user
        user = random.choice(users)
        # select a random post
        post = random.choice(posts)
        comments.append({
            'id': uuid,
            'user_id': user['id'],
            'post_id': post['id'],
            'comment': FAKER.text(),
            'parent_comment_id': None,
            'created_at': FAKER.date_time(),
        })

    print("Done")
    print("Inserting comments into database...", end="")

    # Insert comments into table
    for comment in comments:
        CURSOR.execute("""
            INSERT INTO 
                    comments (id, user_id, post_id, comment, parent_comment_id, created_at) 
                    VALUES (%s, %s, %s, %s, %s, %s)
            """,
            (comment['id'], comment['user_id'], comment['post_id'], comment['comment'], comment['parent_comment_id'], comment['created_at'])
        )

    # Commit changes to database
    CONN.commit()
    print("Done")

def create_likes(users, posts):
    # Add some likes to random posts from users
    print("Creating likes...", end="")
    likes = []
    while len(likes) < 50:
        # select a random user
        user = random.choice(users)
        # select a random post
        post = random.choice(posts)
        # Make sure were not adding the same like twice
        if any(user['id'] == l['user_id'] and post['id'] == l['post_id'] for l in likes):
            continue
        likes.append({
            'user_id': user['id'],
            'post_id': post['id'],
            'created_at': FAKER.date_time(),
        })
    print("Done")
    print("Inserting likes into database...", end="")

    # Insert likes into table
    for like in likes:
        CURSOR.execute("""
            INSERT INTO 
                    likes (user_id, post_id, created_at) 
                    VALUES (%s, %s, %s)
            """,
            (like['user_id'], like['post_id'], like['created_at'])
        )

    # Commit changes to database
    CONN.commit()
    print("Done")

def clear_tables():
    # Clear the tables
    print("Clearing tables...", end="")
    CURSOR.execute("DELETE FROM users_followers")
    CURSOR.execute("DELETE FROM likes")
    CURSOR.execute("DELETE FROM comments")
    CURSOR.execute("DELETE FROM posts")
    CURSOR.execute("DELETE FROM users")
    CONN.commit()
    print("Done")

if __name__ == "__main__":
    import sys
    # Check if the user wants to clear the tables
    if len(sys.argv) > 1 and sys.argv[1] == "clear":
        clear_tables()
    main()