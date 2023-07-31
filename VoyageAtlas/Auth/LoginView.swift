//
//  LoginView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/24/23.
//

import SwiftUI

struct LoginView: View {
    @State private var email: String = ""
    @State private var password: String = ""
    @State private var vm = AuthViewModel()
    @State var onSuccess: () -> Void
    @Binding var isLoggedIn: Bool;
    
    var body: some View {
        VStack {
            GroupBox(label: Label("Login", systemImage: "person.circle.fill")) {
                TextField("Email", text: $email)
                    .autocorrectionDisabled(true)
                    .padding(4)
                SecureField("Password", text: $password) {}.padding(4)
                HStack {
                    Button(action: {
                        vm.login(body: LoginBody(email: email, password: password)) {
                            isLoggedIn.toggle()
                            print("isLoggedIn has been toggled value is now: " + (isLoggedIn ? "true" : "false"))
                        }
                        
                    }) {
                        Text("Sign In")
                    }.buttonStyle(.bordered)
                    Button(action: {}) {
                        Text("Register")
                    }.buttonStyle(.bordered)
                }
                
            }
        }
    }
}

struct LoginView_Previews: PreviewProvider {
    static var previews: some View {
        @State var loggedIn = false;
        LoginView(onSuccess: {}, isLoggedIn: $loggedIn)
    }
}

struct LoginBody: Decodable, Encodable {
    var email: String
    var password: String
}

struct Token: Decodable {
    var bearer: String
    var user: AuthUser
}

struct AuthUser: Decodable {
    var id: String
    var username: String
    var email: String
}
