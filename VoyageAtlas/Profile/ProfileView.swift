//
//  ProfileView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/23/23.
//

import SwiftUI
import AlertToast
import PagerTabStripView

struct ProfileView: View {
    @StateObject private var profilePostFetcher = ProfilePostFetcher(userId: "")
    
    var body: some View {
        VStack {
            HeaderView().background(ignoresSafeAreaEdges: .all)
            ScrollView {
                VStack {
                    // Header
                    HStack {
                        VStack {
                            ProfilePicture(width: 100, height: 100, circleOverlayWidth: 2)
                                .offset(y: -16)
                        }
                        
                        VStack {
                            VStack {
                                Text("Josue Morales")
                                    .font(.headline)
                                Text("@JosueMorales")
                                    .font(.subheadline)
                            }
                            .frame(maxWidth: .infinity, alignment: .trailing)
                            .padding(EdgeInsets(top: 10, leading: 0, bottom: 4, trailing: 12))
                            VStack {
                                Text("My Description Belongs Here")
                                    .font(.subheadline)
                                    .padding(.bottom, 8)
                                HStack {
                                    Text("10 Followers")
                                    Text("10 Following")
                                }
                            }.frame(maxWidth: .infinity, alignment: .leading)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    Divider()
                    
                    if (!profilePostFetcher.isLoading) {
                        PostListView(posts: profilePostFetcher.posts)
                    } else {
                        Spacer()
                    }
                }
                .frame(maxHeight: .infinity)
                .toast(isPresenting: $profilePostFetcher.isLoading) {
                    AlertToast(type: .loading)
                }.refreshable {
                    profilePostFetcher.getUsersPost(userId: "")
                }
            }.padding(.top, -8)
        }

    }
}

struct ProfileView_Previews: PreviewProvider {
    static var previews: some View {
        ProfileView()
    }
}

struct ScrollingProfileView: View {
    var body: some View {
        VStack {
            Image("HeaderAnnapolis")
                .resizable()
                .frame(maxWidth: .infinity, maxHeight: 200)
                .ignoresSafeArea(edges: .top)
            ProfilePicture(width: 200, height: 200)
                .offset(y: -175)
                .padding(.bottom, -250)
            Text("Josue Morales")
                .font(.title)
                .offset(y: -10)

            Text("My Description Belongs Here")
                .font(.subheadline)
                .padding(.bottom, 8)
        }
    }
}
