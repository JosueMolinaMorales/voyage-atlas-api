//
//  AuthViewModel.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/28/23.
//

import Foundation

class AuthViewModel: ObservableObject {
    private let apiUri = "http://localhost:3000"
    func login(body: LoginBody, onSuccess: @escaping () -> Void) {
        guard let url = URL(string: "\(apiUri)/users/login") else { fatalError("Missing URL") }
        let jsonData = try! JSONEncoder().encode(body)
        
        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.httpBody = jsonData
        urlRequest.addValue("application/json", forHTTPHeaderField: "content-type")
        
        let task = URLSession.shared.dataTask(with: urlRequest) { data, response, error in
            guard let data = data, error == nil else {
                print(error?.localizedDescription ?? "No data")
                return
            }
            
            if let token = try? JSONDecoder().decode(Token.self, from: data) {
                // TODO: Update this
                UserDefaults.standard.set(token.bearer, forKey: "token")
                UserDefaults.standard.set(token.user.id, forKey: "userId")
                UserDefaults.standard.set(token.user.username, forKey: "username")
                onSuccess()
                print("Logged In!")
            } else {
                print("Token not retrieved")
            }
        }
        
        task.resume()
    }
    
}
