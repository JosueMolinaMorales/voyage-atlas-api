//
//  CreatePostViewModel.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/28/23.
//

import Foundation

class CreatePostViewModel: ObservableObject {
    private let apiUri = "http://localhost:3000"
    
    func createPost(body: CreatePost, onSuccess: @escaping () -> Void, onError: @escaping (_ status: Int) -> Void) {
        guard let url = URL(string: "\(apiUri)/users/post") else { fatalError("Missing URL") }
        let jsonData = try! JSONEncoder().encode(body)
        let token = UserDefaults.standard.string(forKey: "token") ?? "";
        
        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.httpBody = jsonData
        urlRequest.addValue("application/json", forHTTPHeaderField: "content-type")
        urlRequest.addValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        
        let task = URLSession.shared.dataTask(with: urlRequest) { _, response, error in
            guard error == nil else {
                print(error?.localizedDescription ?? "No data")
                return
            }
            let response = response as? HTTPURLResponse;
            let statusCode = response?.statusCode ?? 0;
            if statusCode == 201 {
                onSuccess()
            } else {
                onError(statusCode)
            }
        }
        
        task.resume()
    }
 
}
