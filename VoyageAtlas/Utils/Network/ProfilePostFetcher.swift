//
//  ProfilePostFetcher.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/27/23.
//

import Foundation

class ProfilePostFetcher: ObservableObject {
    @Published var posts = [Post]()
    @Published var isLoading = true
    var userId: String;
    private let apiUri = "http://localhost:3000"
    
    init(userId: String) {
        self.userId = userId
        getUsersPost(userId: userId)
    }
    
    func getUsersPost(userId: String) {
        isLoading = true
        guard let url = URL(string: "\(apiUri)/users/4fef2aec-4b4f-4354-a32f-d436310385cc/posts") else { fatalError("Missing URL") }

        let token = UserDefaults.standard.string(forKey: "token") ?? "";
        
        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.addValue("application/json", forHTTPHeaderField: "content-type")
        urlRequest.addValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        let task = URLSession.shared.dataTask(with: urlRequest) { data, response, error in
            guard let data = data, error == nil else {
                print(error?.localizedDescription ?? "No data")
                return
            }
            if let posts = try? JSONDecoder().decode([Post].self, from: data) {
                DispatchQueue.main.async {
                    self.isLoading = false
                    self.posts = posts.sorted(by: {p1, p2 in p1.created_at > p2.created_at })
                }
                print(posts)
            } else {
                let response = response as? HTTPURLResponse;
                let statusCode = response?.statusCode ?? 0;
            }
        }
        
        task.resume()
    }
}
