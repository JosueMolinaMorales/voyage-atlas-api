//
//  DiscoverView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/29/23.
//

import SwiftUI

struct DiscoverView: View {
    var body: some View {
        VStack {
            Image(systemName: "sparkles")
                .resizable()
                .frame(width: 100, height: 100)
            Text("Discover Coming Soon")
        }
    }
}

struct DiscoverView_Previews: PreviewProvider {
    static var previews: some View {
        DiscoverView()
    }
}
