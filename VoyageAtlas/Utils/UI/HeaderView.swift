//
//  HeaderView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/29/23.
//

import SwiftUI

struct HeaderView: View {
    var body: some View {
        HStack {
            Image("VoyageBoat")
                .resizable()
                .frame(width: 50, height: 50)
                .padding(.bottom)
        }
        .frame(maxWidth: .infinity)
        .background(VoyageColors.raisinBlack)
    }
}

struct HeaderView_Previews: PreviewProvider {
    static var previews: some View {
        HeaderView()
    }
}
