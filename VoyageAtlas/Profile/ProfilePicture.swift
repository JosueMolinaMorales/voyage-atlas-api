//
//  ProfilePicture.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/23/23.
//

import SwiftUI

struct ProfilePicture: View {
    var width: CGFloat
    var height: CGFloat
    var shadowRadius: CGFloat = 7
    var circleOverlayWidth: CGFloat = 4
    var body: some View {
        Image("ProfilePic")
            .resizable()
            .clipShape(Circle())
            .overlay {
                Circle().stroke(VoyageColors.orange, lineWidth: circleOverlayWidth)
            }
            .aspectRatio(contentMode: .fit)
            .frame(width: width, height: height)
            .shadow(radius: shadowRadius)
    }
}

struct ProfilePicture_Previews: PreviewProvider {
    static var previews: some View {
        ProfilePicture(width: 200, height: 200)
    }
}
