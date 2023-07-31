//
//  PostListView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/23/23.
//

import SwiftUI

struct PostListView: View {
    @State var posts: [Post]
    var body: some View {
        ForEach(posts) {post in
            VStack {
                PostView(post: post)
                Divider()
            }
        }
    }
}

struct PostListView_Previews: PreviewProvider {
    static var previews: some View {
        PostListView(posts: [])
    }
}
