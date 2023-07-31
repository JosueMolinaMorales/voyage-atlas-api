//
//  PostTypes.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/27/23.
//

import Foundation

struct CreatePost: Encodable {
    var title: String = ""
    var location: String = ""
    var content: String = ""
}

struct Post: Decodable, Identifiable {
    var title: String
    var location: String
    var content: String
    var id: String
    var author: String
    var created_at: Double
    
    func convertCreatedAt() -> String {
        let date = Date(timeIntervalSince1970: self.created_at)
        let dateFormatter = DateFormatter()
        dateFormatter.timeZone = TimeZone(abbreviation: "EST") //Set timezone that you want
        dateFormatter.locale = NSLocale.current
        dateFormatter.dateFormat = "HH:mm yyyy-MM-dd" //Specify your format that you want
        let strDate = dateFormatter.string(from: date)
        return strDate
    }
}
