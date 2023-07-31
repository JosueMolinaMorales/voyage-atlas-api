//
//  HomeView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/25/23.
//

import SwiftUI
import FloatingButton

struct HomeView: View {
    @State private var showCreatePost = false
    @State var onCreatePostSuccess: () -> Void
    
    var body: some View {
        TabView {
            NavigationStack {
                HeaderView()
                ZStack(alignment: .bottomTrailing) {
                    List(0..<100) { i in
                        Text("Item \(i)")
                    }
                    
                    Button {
                        showCreatePost.toggle()
                    } label: {
                        Image(systemName: "plus")
                            .font(.title.weight(.semibold))
                            .padding()
                            .background(Color.pink)
                            .foregroundColor(.white)
                            .clipShape(Circle())
                            .shadow(radius: 4, x: 0, y: 4)

                    }
                    .padding()
                    .sheet(isPresented: $showCreatePost ) {
                        CreatePostView(onSuccess: onCreatePostSuccess)
                            .presentationDragIndicator(.visible)
                            .presentationCompactAdaptation(.popover)
                    }
                }
            }
            .tabItem {
                Label("Home", systemImage: "house")
            }
            DiscoverView()
                .tabItem {
                    Label("Discover", systemImage: "sparkles")
                }
            ProfileView()
            .tabItem {
                Label("Profile", systemImage: "person.crop.circle")
            }
        }
    }
}

struct HomeView_Previews: PreviewProvider {
    static var previews: some View {
        HomeView() {
            
        }
    }
}
