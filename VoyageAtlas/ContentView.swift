//
//  ContentView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/23/23.
//

import SwiftUI
import AlertToast

struct ContentView: View {
    @State private var selection: String? = nil
    @State private var isLoggedIn = false;
    @State private var showSnackbar = false
    @State private var snackbarType: SnackbarType = SnackbarType.Error
    @State private var snackbarTitle = ""
    @State private var snackbarMsg = ""
    @State private var showLoginView = false;

    init() {
        let token = UserDefaults.standard.string(forKey: "token") ?? ""
        self.isLoggedIn = !token.isEmpty
    }
    
    var body: some View {
        NavigationStack {
            WelcomeView(onSignInWithEmail: {
                showLoginView = true
            })
            .navigationDestination(isPresented: $showLoginView) {
                LoginView(onSuccess: {
                    showSnackbar = true
                    snackbarMsg = "You have been logged in"
                    snackbarType = SnackbarType.Success
                    snackbarTitle = "Logged in!"
                }, isLoggedIn: $isLoggedIn)
                    .navigationBarBackButtonHidden(true)
                    .navigationDestination(isPresented: $isLoggedIn) {
                        HomeView() {
                            snackbarTitle = "Post Created!"
                            snackbarMsg = "Your post has been created."
                            snackbarType = SnackbarType.Success
                            showSnackbar = true
                        }
                        .navigationBarBackButtonHidden(true)
                        .toast(isPresenting: $showSnackbar, duration: 5){
                            AlertToast(
                                displayMode: snackbarType == SnackbarType.Error ? .banner(.pop) : .hud,
                                type: snackbarType == SnackbarType.Error ? .error(Color.red) : .complete(Color.green),
                                title: snackbarTitle,
                                subTitle: snackbarMsg
                            )
                        }
                    }
                }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}

enum Views: Hashable {
    case Login
    case Home
}
