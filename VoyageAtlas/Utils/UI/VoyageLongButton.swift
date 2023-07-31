//
//  VoyageLongButton.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/29/23.
//

import SwiftUI

struct VoyageLongButton: View {
    var text: String
    var action: () -> Void
    var body: some View {
        Button{action()} label: {
            Text(text)
                .frame(width: 200)
                .font(.body)
                .fontWeight(.heavy)
                .foregroundColor(Color.white)
                .padding(10)
                .background(VoyageColors.orange)
                .cornerRadius(10)
        }.buttonStyle(PlainButtonStyle())
    }
}

struct VoyageLongButton_Previews: PreviewProvider {
    static var previews: some View {
        VoyageLongButton(text: "Sign In", action: {})
    }
}
